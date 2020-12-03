use brainfuck2c::Token;
use std::{collections::btree_map::Values, fmt};
pub fn brainfuck_codegen(input: &Vec<Token>) -> String {
    let mut codegen = CodeGen::new();
    codegen.add_import("stdio.h");
    let main_func = codegen.add_function("main", "int");
    let scope = &mut main_func.scope;
    scope.add_line(Line::new_arrdef("memory", "int", 30000));
    scope.add_line(Line::new_vardef("memPointer", "int", Value::Int(0)));
    &mut brainfuck_codegen_rec(input, scope);
    codegen.to_string()
}
fn brainfuck_codegen_rec(input: &Vec<Token>, scope: &mut Scope) {
    for token in input {
        let INDEX_MEMORY_NOW: ObjectRef = ObjectRef::ListIndex(ListIndex {
            name: "memory".to_string(),
            index: Index::VarRef("memPointer".to_string()),
        });
        let MEMPOINTER: ObjectRef = ObjectRef::VarRef("memPointer".to_string());
        match token {
            Token::ChangeMem(by) => scope.add_line(Line::new_varinc(INDEX_MEMORY_NOW, *by)),
            Token::SetMemTo(to) => scope.add_line(Line::VarSet {
                name: INDEX_MEMORY_NOW,
                to: Value::Int(*to),
            }),
            Token::MovePointer(by) => scope.add_line(Line::new_varinc(MEMPOINTER, *by)),
            Token::Print => scope.add_line(Line::new_funccall("putchar", vec![INDEX_MEMORY_NOW])),
            Token::Loop(code) => {
                let mut nscope = Scope::new();
                brainfuck_codegen_rec(code, &mut nscope);
                scope.add_line(Line::While {
                    condition: Condition::NotEq(INDEX_MEMORY_NOW, ObjectRef::Value(Value::Int(0))),
                    scope: nscope,
                });
            }
            Token::None => (),
            _ => unimplemented!(),
        };
    }
}
pub struct CodeGen {
    imports: Vec<Import>,
    functions: Vec<Function>,
}
impl CodeGen {
    fn new() -> Self {
        CodeGen {
            functions: vec![],
            imports: vec![],
        }
    }
    fn add_function(&mut self, name: &str, return_type: &str) -> &mut Function {
        self.functions.push(Function::new(name, return_type));
        self.functions.last_mut().unwrap()
    }
    fn add_import(&mut self, name: &str) {
        self.imports.push(Import::new(name));
    }
}
impl fmt::Display for CodeGen {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}\n{}",
            self.imports
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            self.functions
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub struct Function {
    name: String,
    return_type: String,
    scope: Scope,
}
impl Function {
    fn new(name: &str, return_type: &str) -> Self {
        Function {
            name: name.to_string(),
            return_type: return_type.to_string(),
            scope: Scope::new(),
        }
    }
}
impl fmt::Display for Function {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{} {}() {{\n{}\n}}",
            self.return_type,
            self.name,
            self.scope.to_string()
        )
    }
}
pub struct Import {
    name: String,
}
impl Import {
    fn new(name: &str) -> Self {
        Import {
            name: name.to_string(),
        }
    }
}
impl fmt::Display for Import {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "#include <{}>", self.name)
    }
}
pub struct Scope {
    code: Code,
}
impl Scope {
    fn new() -> Self {
        Scope { code: Code::new() }
    }
    fn add_line(&mut self, line: Line) {
        self.code.lines.push(line);
    }
}
impl fmt::Display for Scope {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            ("\t".to_string() + &self.code.to_string())
                .split("\n")
                .collect::<Vec<&str>>()
                .join("\n\t")
        )
    }
}
pub struct Code {
    lines: Vec<Line>,
}
impl Code {
    fn new() -> Self {
        Code { lines: vec![] }
    }
}
impl fmt::Display for Code {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            self.lines
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
enum ObjectRef {
    ListIndex(ListIndex),
    VarRef(VarRef),
    Value(Value),
}
impl fmt::Display for ObjectRef {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectRef::VarRef(vr) => write!(fmt, "{}", vr),
            ObjectRef::ListIndex(li) => write!(fmt, "{}[{}]", li.name, li.index.to_string()),
            ObjectRef::Value(v) => write!(fmt, "{}", v),
        }
    }
}
struct ListIndex {
    name: String,
    index: Index,
}
type VarRef = String;
enum Index {
    Int(i32),
    VarRef(VarRef),
}
impl fmt::Display for Index {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Index::VarRef(vr) => write!(fmt, "{}", vr),
            Index::Int(i) => write!(fmt, "{}", i.to_string()),
        }
    }
}
enum Value {
    String(String),
    Int(i32),
    Bool(bool),
}
impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(fmt, "\"{}\"", s),
            Value::Int(i) => write!(fmt, "{}", i),
            Value::Bool(b) => match b {
                true => write!(fmt, "true"),
                false => write!(fmt, "false"),
            },
        }
    }
}
enum Condition {
    NotEq(ObjectRef, ObjectRef),
    Eq(ObjectRef, ObjectRef),
}
impl fmt::Display for Condition {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::Eq(v1, v2) => write!(fmt, "{} == {}", v1, v2),
            Condition::NotEq(v1, v2) => write!(fmt, "{} != {}", v1, v2),
        }
    }
}
enum Line {
    VarDef {
        name: String,
        var_type: String,
        value: Value,
    },
    ArrDef {
        name: String,
        var_type: String,
        len: i32,
    },
    VarInc {
        name: ObjectRef,
        by: i32,
    },
    VarSet {
        name: ObjectRef,
        to: Value,
    },
    FuncCall {
        name: String,
        params: Vec<ObjectRef>,
    },
    While {
        condition: Condition,
        scope: Scope,
    },
}
impl Line {
    fn new_vardef(name: &str, var_type: &str, value: Value) -> Self {
        Line::VarDef {
            name: name.to_string(),
            var_type: var_type.to_string(),
            value: value,
        }
    }
    fn new_arrdef(name: &str, var_type: &str, len: i32) -> Self {
        Line::ArrDef {
            name: name.to_string(),
            var_type: var_type.to_string(),
            len: len,
        }
    }
    fn new_varinc(name: ObjectRef, by: i32) -> Self {
        Line::VarInc { name: name, by: by }
    }
    fn new_funccall(name: &str, params: Vec<ObjectRef>) -> Self {
        Line::FuncCall {
            name: name.to_string(),
            params,
        }
    }
}
impl fmt::Display for Line {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::VarDef {
                name,
                var_type,
                value,
            } => write!(fmt, "{} {} = {};", var_type, name, value),
            Line::ArrDef {
                name,
                var_type,
                len,
            } => write!(fmt, "{} {}[{}];", var_type, name, len),
            Line::VarInc { name, by } => write!(fmt, "{} += {};", name, by),
            Line::FuncCall { name, params } => write!(
                fmt,
                "{}({});",
                name,
                params
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Line::While { condition, scope } => {
                write!(fmt, "while ({}) {{\n{}\n}}", condition, scope)
            }
            Line::VarSet { name, to } => write!(fmt, "{} = {};", name, to),
        }
    }
}

use std::fs;

fn main() {
    println!("Reading ROM.bf");
    let rom = fs::read_to_string("ROM.bf").unwrap();
    let parsed = brainfuck_parser(rom);
    let mut memory = [0; 30000];
    println!("Parsed!");
    brainfuck_interpreter(&parsed, &mut memory, &mut 0);
    println!("{}", prettify_memory(&memory[..]));
}
#[derive(Debug, Clone)]
enum Token {
    MovePointer(i32),
    ChangeMem(i32),
    Loop(Vec<Token>),
    Input,
    Print,
}
fn brainfuck_parser(input: String) -> Vec<Token> {
    let mut stack: Vec<Vec<Token>> = vec![vec![]];
    for char in input.split("").filter(|n| *n != "") {
        if let Some(tok) = match char {
            "+" => Some(Token::ChangeMem(1)),
            "-" => Some(Token::ChangeMem(-1)),
            ">" => Some(Token::MovePointer(1)),
            "<" => Some(Token::MovePointer(-1)),
            "," => Some(Token::Input),
            "." => Some(Token::Print),
            "[" => {
                stack.insert(0, vec![]);
                None
            }
            "]" => Some(Token::Loop(stack.remove(0))),
            _ => None,
        } {
            stack[0].push(tok);
        }
    }
    stack[0].clone()
}
fn brainfuck_interpreter(input: &Vec<Token>, memory: &mut [i32; 30000], mem_pointer: &mut usize) {
    for token in input {
        match token {
            Token::ChangeMem(by) => memory[*mem_pointer] += by,
            Token::MovePointer(by) => {
                if by < &0 {
                    *mem_pointer -= by.abs() as usize
                } else {
                    *mem_pointer += by.abs() as usize
                }
            }
            Token::Print => print!("{}", (memory[*mem_pointer] as u8) as char),
            Token::Loop(code) => {
                while memory[*mem_pointer] != 0 {
                    brainfuck_interpreter(&code, memory, mem_pointer)
                }
            }
            _ => unimplemented!(),
        };
    }
}
fn prettify_memory(mem: &[i32]) -> String {
    mem.into_iter()
        .rev()
        .skip_while(|&x| x == &0)
        .collect::<Vec<&i32>>()
        .into_iter()
        .rev()
        .map(|m| format!("[{}]", m))
        .collect::<Vec<String>>()
        .join("")
}

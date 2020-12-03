struct Stream<T> {
    items: Vec<T>,
}
impl<T> Stream<T> {
    fn peek(&self) -> Option<&T> {
        if self.items.len() > 0 {
            Some(&self.items[0])
        } else {
            None
        }
    }
    fn new(items: Vec<T>) -> Self {
        Self { items }
    }
    fn peekn(&self, n: usize) -> Option<&T> {
        if self.items.len() > n {
            Some(&self.items[n])
        } else {
            None
        }
    }
    fn next(&mut self) -> Option<T> {
        if self.items.len() > 0 {
            Some(self.items.remove(0))
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    MovePointer(i32),
    ChangeMem(i32),
    SetMemTo(i32),
    Loop(Vec<Token>),
    Input,
    Print,
    None,
}
fn add(u: usize, i: i32) -> usize {
    if i < 0 {
        u - i.abs() as usize
    } else {
        u + i.abs() as usize
    }
}
pub fn brainfuck_parser(input: String) -> Vec<Token> {
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
pub fn brainfuck_optimizer(input: Vec<Token>) -> Vec<Token> {
    let mut data = Stream::new(input);
    let mut output: Vec<Token> = vec![];
    if let Token::Loop(_) = data.peek().unwrap() {
        data.next();
    }
    while data.peek().is_some() {
        if *data.peek().unwrap() == Token::Loop(vec![Token::ChangeMem(-1)]) {
            output.push(Token::SetMemTo(0));
            data.next();
        } else if let Token::ChangeMem(_) = *data.peek().unwrap() {
            let mut curr_change = 0;
            while let Some(Token::ChangeMem(by)) = data.peek() {
                curr_change += by;
                data.next();
            }
            output.push(Token::ChangeMem(curr_change));
        } else if let Token::MovePointer(_) = *data.peek().unwrap() {
            let mut curr_change = 0;
            while let Some(Token::MovePointer(by)) = data.peek() {
                curr_change += by;
                data.next();
            }
            output.push(Token::MovePointer(curr_change));
        } else if let Token::Loop(code) = data.peek().unwrap() {
            output.push(Token::Loop(brainfuck_optimizer(code.clone())));
            data.next();
        } else {
            output.push(data.next().unwrap().clone());
        }
    }
    output
}
pub fn brainfuck_interpreter(
    input: &Vec<Token>,
    memory: &mut [i32; 30000],
    mem_pointer: &mut usize,
    print: bool,
) {
    for token in input {
        match token {
            Token::ChangeMem(by) => memory[*mem_pointer] += by,
            Token::MovePointer(by) => *mem_pointer = add(*mem_pointer, *by),
            Token::Print => {
                if print {
                    print!("{}", (memory[*mem_pointer] as u8) as char)
                }
            }
            Token::Loop(code) => {
                while memory[*mem_pointer] != 0 {
                    brainfuck_interpreter(&code, memory, mem_pointer, print)
                }
            }
            Token::None => (),
            Token::SetMemTo(i) => memory[*mem_pointer] = *i,
            _ => unimplemented!(),
        };
    }
}
pub fn prettify_memory(mem: &[i32]) -> String {
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

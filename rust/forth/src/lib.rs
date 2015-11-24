use std::collections::VecDeque;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    pub instructions: VecDeque<Box<Instruction>>,
    pub stack: Vec<i32>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub trait Instruction {
    fn eval(&self, forth: &mut Forth) -> ForthResult;
}

struct Add;
impl Instruction for Add {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        forth.binary_operation(|x, y| Ok(x + y))
    }
}
struct Sub;
impl Instruction for Sub {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        forth.binary_operation(|x, y| Ok(x - y))
    }
}
struct Mul;
impl Instruction for Mul {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        forth.binary_operation(|x, y| Ok(x * y))
    }
}
struct Div;
impl Instruction for Div {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        forth.binary_operation(|x, y|
            if y == 0 { Err(Error::DivisionByZero) } else { Ok(x / y) }
        )
    }
}
struct Dup;
impl Instruction for Dup {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        match forth.stack.pop() {
            Some(x) => { forth.stack.push(x); forth.stack.push(x); Ok(()) }
            None => Err(Error::StackUnderflow)
        }
    }
}
struct Drop;
impl Instruction for Drop {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        match forth.stack.pop() {
            Some(_) => { Ok(()) }
            None => Err(Error::StackUnderflow)
        }
    }
}
struct Number {
    n: i32
}
impl Instruction for Number {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        forth.stack.push(self.n);
        Ok(())
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![], instructions: VecDeque::new() }
    }

    pub fn format_stack(&self) -> String {
        self.stack.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn binary_operation<F>(&mut self, operation: F) -> ForthResult
        where F : Fn(i32, i32) -> Result<i32, Error>
    {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(y), Some(x)) => {
                match operation(x, y) {
                    Ok(answer) => { self.stack.push(answer); Ok(()) }
                    Err(e) => Err(e)
                }
            }
            (_, _) => Err(Error::StackUnderflow)
        }
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let tokens: Vec<&str> = input.split(|c: char| !c.is_alphanumeric() && !"-*+/".contains(c)).collect();
        self.process_tokens(&tokens)
    }

    fn process_tokens(&mut self, tokens: &Vec<&str>) -> ForthResult {
        self.instructions = tokens.iter().map(|token| Self::build_operation(token)).collect();
        loop {
            match self.instructions.pop_front() {
                Some(instruction) => { try!(instruction.eval(self)); }
                None => { break; }
            }
        }
        Ok(())
    }

    fn build_operation(token: &str) -> Box<Instruction> {
        match token.to_lowercase().as_ref() {
            "+" => Box::new(Add),
            "-" => Box::new(Sub),
            "*" => Box::new(Mul),
            "/" => Box::new(Div),
            "dup" => Box::new(Dup),
            "drop" => Box::new(Drop),
            num @ _ => Box::new(Number { n: num.parse::<i32>().ok().unwrap() })
        }
    }
}

use std::collections::VecDeque;
use std::collections::HashMap;
use std::iter::FromIterator;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    pub instructions: VecDeque<String>,
    pub stack: Vec<i32>,
    pub custom_instructions: HashMap<String, CustomInstructionEvaluator>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

const BEGIN_STRING: &'static str = ":";
const END_STRING: &'static str = ";";

enum Token<'a> {
    Number(i32),
    Token(&'a str),
    Empty,
    Begin,
    End
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
        // This somewhat awkward syntax is necessary because we last() will borrow
        // the stack and we can't push onto it while it is borrowed.
        let last = match forth.stack.last() { Some(x) => Some(*x), None => None };
        if let Some(x) = last { forth.stack.push(x); Ok(()) } else { Err(Error::StackUnderflow) }
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
struct Swap;
impl Instruction for Swap {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        let stack_size = forth.stack.len();
        if stack_size >= 2 {
            forth.stack.swap(stack_size - 1, stack_size - 2);
            Ok(())
        }
        else {
            Err(Error::StackUnderflow)
        }
    }
}
struct Over;
impl Instruction for Over {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        let stack_size = forth.stack.len();
        if stack_size >= 2 {
            let x = forth.stack[stack_size - 2];
            forth.stack.push(x);
            Ok(())
        }
        else {
            Err(Error::StackUnderflow)
        }
    }
}
struct NoOp;
impl Instruction for NoOp {
    fn eval(&self, _: &mut Forth) -> ForthResult { Ok(()) }
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
// You need a custom instruction reader and a separate custom instruction evaluator
pub struct CustomInstructionReader;
impl CustomInstructionReader {
    fn read_name(instructions: &mut VecDeque<String>) -> Result<String, Error> {
        if let Some(instruction) = instructions.pop_front() {
            match Forth::tokenize(instruction.as_ref()) {
                Token::Token(token) => { return Ok(token.to_lowercase()); }
                _ => { return Err(Error::InvalidWord); }
            }
        } else {
            return Err(Error::InvalidWord);
        }
    }

    fn read_expansion(instructions: &mut VecDeque<String>) -> Result<Vec<String>, Error> {
        let mut expansion: Vec<String> = vec![];
        loop {
            if let Some(instruction) = instructions.pop_front() {
                match instruction.as_ref() {
                    END_STRING => { return Ok(expansion); }
                    _ => { expansion.push(instruction.to_string()) }
                }
            } else {
                return Err(Error::InvalidWord);
            }
        }
    }
}
impl Instruction for CustomInstructionReader {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        match (Self::read_name(&mut forth.instructions), Self::read_expansion(&mut forth.instructions)) {
            (Ok(name), Ok(expansion)) => {
                forth.custom_instructions.insert(name, CustomInstructionEvaluator::new(expansion));
                Ok(())
            }
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e)
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomInstructionEvaluator {
    expansion: Vec<String>
}
impl CustomInstructionEvaluator {
    fn new(expansion: Vec<String>) -> CustomInstructionEvaluator {
        CustomInstructionEvaluator { expansion: expansion }
    }
}
impl Instruction for CustomInstructionEvaluator {
    fn eval(&self, forth: &mut Forth) -> ForthResult {
        for instruction in self.expansion.iter().cloned().rev() {
            forth.instructions.push_front(instruction)
        }
        Ok(())
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![], instructions: VecDeque::new(), custom_instructions: HashMap::new() }
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
        self.instructions = Self::split(input);
        self.interpret_instructions()
    }

    fn interpret_instructions(&mut self) -> ForthResult {
        loop {
            match self.instructions.pop_front() {
                Some(instruction) => {
                    match self.build_operation(instruction) {
                        Ok(op) => { try!(op.eval(self)) }
                        Err(e) => { return Err(e); }
                    }
                }
                None => { return Ok(()); }
            }
        }
    }

    fn build_token_operation(&self, token: &str) -> Result<Box<Instruction>, Error> {
        if let Some(instruction) = self.custom_instructions.get(token).cloned() {
            return Ok(Box::new(instruction));
        }
        match token {
            "+" => Ok(Box::new(Add)),
            "-" => Ok(Box::new(Sub)),
            "*" => Ok(Box::new(Mul)),
            "/" => Ok(Box::new(Div)),
            "dup" => Ok(Box::new(Dup)),
            "drop" => Ok(Box::new(Drop)),
            "swap" => Ok(Box::new(Swap)),
            "over" => Ok(Box::new(Over)),
            _ => Err(Error::UnknownWord)
        }
    }

    fn build_operation(&self, input: String) -> Result<Box<Instruction>, Error> {
        match Self::tokenize(input.as_ref()) {
            Token::Number(n) => Ok(Box::new(Number { n: n })),
            Token::Begin => Ok(Box::new(CustomInstructionReader)),
            Token::End => Err(Error::InvalidWord),
            Token::Empty => Ok(Box::new(NoOp)),
            Token::Token(token) => self.build_token_operation(token.to_lowercase().as_ref())
        }
    }

    fn split<T>(input: &str) -> T
        where T: FromIterator<String>
    {
        input.split(|c: char| c.is_whitespace() || c.is_control())
            .map(|s| s.to_string())
            .collect::<T>()
    }

    fn tokenize<'a>(input: &'a str) -> Token<'a> {
        match input {
            BEGIN_STRING => { return Token::Begin; }
            END_STRING => { return Token::End; }
            "" => { return Token::Empty; }
            _ => { }
        }

        if let Ok(n) = input.parse::<i32>() { return Token::Number(n); }
        return Token::Token(input);
    }
}

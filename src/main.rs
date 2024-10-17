mod parser;
mod codegen;

use std::path::PathBuf;
use std::fs;
use std::fmt;
use clap::{ValueEnum, Parser};

#[derive(Debug, Copy, Clone, ValueEnum)]
enum OutputLang {
    Forth,
}

#[derive(Debug, Clone, Parser)]
struct Cli {
    input: PathBuf,
    #[arg(short, long)]
    debug: bool,
    //output: PathBuf,
    //output_type: OutputLang,
}

#[derive(Clone, Debug)]
enum FValue {
    Int(i32),
    Str(String),
}

impl ToString for FValue {
    fn to_string(&self) -> String {
        match self {
            Self::Int(i) => format!("{i}"),
            Self::Str(s) => s.clone(),
        }
    }
}

struct Stack(Vec<FValue>);

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[ ")?;
        for item in &self.0 {
            f.write_fmt(format_args!("{:?} ", item.to_string()))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl Stack {
    fn pop_int(&mut self) -> Option<i32> {
        let FValue::Int(i) = self.0.pop()? else { return None };
        return Some(i);
    }
    fn pop(&mut self) -> FValue { self.0.pop().unwrap() }
    fn push(&mut self, v: FValue) { self.0.push(v) }
    fn push_int(&mut self, i: i32) { self.0.push(FValue::Int(i)) }
    fn push_str(&mut self, s: String) { self.0.push(FValue::Str(s)) }
    fn op2_to_1(&mut self, mut op: impl FnMut(i32, i32) -> i32) {
        let v2 = self.pop_int().unwrap();
        let v1 = self.pop_int().unwrap();
        self.push_int(op(v1, v2));
    }
}

use crate::parser::Token;
fn main() {
    let args = Cli::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let tokens = parser::parse_ignore_comments(&input);
    let mut pc = 0usize;
    let mut stack = Stack(Vec::new());
    loop {
        let Some(tok) = tokens.get(pc) else { break };
        if args.debug { dbg!(tok); }
        match tok {
            Token::Comment(_) => (),
            Token::Str(s) => stack.push_str(s.clone()),
            Token::Int(i) => stack.push_int(*i),
            Token::Add => stack.op2_to_1(|v1, v2| v1 + v2),
            Token::Sub => stack.op2_to_1(|v1, v2| v1 - v2),
            Token::Mul => stack.op2_to_1(|v1, v2| v1 * v2),
            Token::Div => stack.op2_to_1(|v1, v2| v1 / v2),
            Token::Mod => stack.op2_to_1(|v1, v2| v1 % v2),
            Token::Equal => stack.op2_to_1(|v1, v2| (v1 == v2) as i32),
            Token::LargerThan => stack.op2_to_1(|v1, v2| (v1 > v2) as i32 ),
            Token::Dup => {
                let tmp = stack.pop();
                stack.push(tmp.clone());
                stack.push(tmp);
            },
            Token::Swap => {
                let v1 = stack.pop();
                let v2 = stack.pop();
                stack.push(v1);
                stack.push(v2);
            },
            Token::Cjump => {

                let offset = stack.pop_int().unwrap();
                let condition = stack.pop_int().unwrap();


                if condition != 0 {
                    pc = (pc as i32 + offset - 1) as usize;
                }
            },
            Token::Print => print!("{}", stack.pop().to_string()),
            Token::Newline => println!(),
        }
        if args.debug { dbg!(&stack); }
        pc += 1;
    }
}

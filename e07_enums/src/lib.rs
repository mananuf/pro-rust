use crate::{interpreter::eval, lexer::tokenize, parser::parse, types::CalcState};

pub mod types;
pub mod lexer;
pub mod parser;
pub mod interpreter;

pub struct MiniCalc {
    pub state: CalcState,
}

impl MiniCalc {
    pub fn new() -> Self {
        Self { state: CalcState::Idle }
    }

    pub fn input(&mut self, source: &str) {
        self.state = CalcState::Ready(source.to_string());
    }

    pub fn run(&mut self) {
        match &self.state {
            CalcState::Ready(input) => {
                let tokens = tokenize(&input);
                match parse(&tokens) {
                    Ok(expr) => {
                        let result = eval(&expr);
                        self.state = CalcState::Finished(result);
                    },
                    Err(e) => {
                        self.state = CalcState::Error(e);
                    }
                }
            },
            _ => {
                self.state = CalcState::Error("cannot run in this state".into())
            }
        }
    }
}
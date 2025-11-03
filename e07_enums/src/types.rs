#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>), // Using Box<Expr> makes this recursive enum possible (otherwise size would be infinite).
    Sub(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum CalcState {
    Idle,
    Ready(String),
    Error(String),
    Finished(i64),
}
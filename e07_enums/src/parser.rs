use crate::types::{Expr, Token};

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    if let Some(Token::Number(a)) = tokens.get(0) {
        match tokens.get(1) {
            Some(Token::Plus) => {
                if let Some(Token::Number(b)) = tokens.get(2) {
                    return Ok(Expr::Add(Box::new(Expr::Number(*a)), Box::new(Expr::Number(*b))));
                }
            }
            Some(Token::Minus) => {
                if let Some(Token::Number(b)) = tokens.get(2) {
                    return Ok(Expr::Sub(Box::new(Expr::Number(*a)), Box::new(Expr::Number(*b))));
                }
            }
            _ => return Ok(Expr::Number(*a)),
        }
    }

    Err("Invalid Expression".into())
}

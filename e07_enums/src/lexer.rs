use crate::types::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    for ch in input.chars() {
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            d if d.is_digit(10) => {
                tokens.push(Token::Number(d.to_digit(10).unwrap() as i64))
            }
            _ => (),
        }
    }
    tokens.push(Token::EOF);
    tokens
}

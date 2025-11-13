use crate::types::PaymentProcessor;

#[derive(Debug)]
pub struct Card {
    pub balance: f64,
}

#[derive(Debug, Default)]
pub struct CardBuilder {
    pub balance: Option<f64>,
}

impl Card {
    pub fn new() -> CardBuilder {
        CardBuilder {
            ..Default::default()
        }
    }
}

impl CardBuilder {
    pub fn balance(mut self, balance: f64) -> Self {
        self.balance = Some(balance);
        self
    }

    pub fn build(&self) -> Card {
        Card {
            balance: self.balance.unwrap_or_default(),
        }
    }
}

impl PaymentProcessor for Card {
    fn authorize(&self, amount: f64) -> bool {
        self.balance >= amount
    }

    fn capture(&self, amount: f64) {
        println!("current {amount}")
    }

    fn refund(&mut self, amount: f64) {
        self.balance += amount;
        println!("refunded {amount}\nnew balance: {}", self.balance)
    }
}

use crate::types::PaymentProcessor;

#[derive(Debug)]
pub struct Paypal {
    pub balance: f64,
}

#[derive(Debug, Default)]
pub struct PaypalBuilder {
    pub balance: Option<f64>,
}

impl Paypal {
    pub fn new() -> PaypalBuilder {
        PaypalBuilder {
            ..Default::default()
        }
    }
}

impl PaypalBuilder {
    pub fn balance(mut self, balance: f64) -> Self {
        self.balance = Some(balance);
        self
    }

    pub fn build(&self) -> Paypal {
        Paypal {
            balance: self.balance.unwrap_or_default(),
        }
    }
}

impl PaymentProcessor for Paypal {
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

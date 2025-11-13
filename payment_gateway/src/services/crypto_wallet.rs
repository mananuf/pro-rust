use crate::types::PaymentProcessor;

#[derive(Debug)]
pub struct CryptoWallet {
    pub balance: f64,
}

#[derive(Debug, Default)]
pub struct CryptoWalletBuilder {
    pub balance: Option<f64>,
}

impl CryptoWallet {
    pub fn new() -> CryptoWalletBuilder {
        CryptoWalletBuilder {
            ..Default::default()
        }
    }
}

impl CryptoWalletBuilder {
    pub fn balance(mut self, balance: f64) -> Self {
        self.balance = Some(balance);
        self
    }

    pub fn build(&self) -> CryptoWallet {
        CryptoWallet {
            balance: self.balance.unwrap_or_default(),
        }
    }
}

impl PaymentProcessor for CryptoWallet {
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

use core::fmt;
use std::fmt::{Debug, Formatter};

pub trait PaymentProcessor {
    fn authorize(&self, amount: f64) -> bool;
    fn capture(&self, amount: f64);
    fn refund(&mut self, amount: f64);
}

impl Debug for dyn PaymentProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PaymentProcessor")
    }
}

use crate::types::PaymentProcessor;

// #[derive(Debug)]
// pub struct PaymentService<T: PaymentProcessor + std::fmt::Debug> {
//     pub processor: T,
// }

// impl<T: PaymentProcessor + std::fmt::Debug> PaymentService<T> {
//     pub fn new(processor: T) -> Self {
//         Self { processor }
//     }
// }

//dynamic for multiple types

type Processor = Box<dyn PaymentProcessor>;
#[derive(Debug)]
pub struct PaymentService {
    pub processor: Processor,
}

impl PaymentService {
    pub fn new(processor: Processor) -> Self {
        Self { processor }
    }
}

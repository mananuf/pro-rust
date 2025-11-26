pub trait Asset {
    fn symbol(&self) -> &str;
}

pub trait PriceFeed<A: Asset> {
    fn get_price(&self, asset: &A) -> f64;
}

pub trait Strategy<A: Asset> {
    fn execute(&self, asset: &A, price: f64);
}

pub struct Trader<A: Asset, F: PriceFeed<A>, S: Strategy<A>> {
    pub asset: A,
    pub feed: F,
    pub strategy: S,
}


pub struct Stock;
pub struct Crypto;
pub struct Forex;

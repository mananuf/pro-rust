use payment_gateway::services::{
    card::Card, crypto_wallet::CryptoWallet, payment_service::PaymentService, paypal::Paypal,
};

fn main() {
    let paypal: Box<Paypal> = Box::new(Paypal::new().balance(10.into()).build());
    let _card: Box<Card> = Box::new(Card::new().build());
    let _crypto_wallet: Box<CryptoWallet> = Box::new(CryptoWallet::new().balance(12.6).build());

    let mut payment_service = PaymentService::new(paypal);
    let auth = payment_service.processor.authorize(12.into());
    println!("{auth}");
    let _ = payment_service.processor.capture(12.into());
    let _ = payment_service.processor.refund(12.into());
}

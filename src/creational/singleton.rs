#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref WALLET: Mutex<Wallet> = Mutex::new(Wallet::new());
}

#[derive(Debug)]
pub struct Wallet {
    amount: f64,
}

impl Wallet {
    fn new() -> Self {
        println!("Creating a wallet");
        Wallet { amount: 0f64 }
    }

    fn set_amount(&mut self, amount: f64) {
        println!("Setting wallet amount: {}", amount);
        self.amount = amount
    }

    fn print_amount(&self) {
        println!("Wallet amount: {}", self.amount);
    }

    fn get() -> MutexGuard<'static, Wallet> {
        WALLET.lock().unwrap()
    }
}

pub fn main() {
    Wallet::get().print_amount();
    Wallet::get().set_amount(42f64);
    Wallet::get().print_amount();
}

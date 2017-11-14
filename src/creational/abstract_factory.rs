use std::option::*;

pub trait Coin {
    fn spend(self: Box<Self>);
}

pub struct BitcoinCoin(u32);

impl BitcoinCoin {
    fn new(amount: u32) -> Self {
        BitcoinCoin(amount)
    }
}

impl Coin for BitcoinCoin {
    fn spend(self: Box<Self>) {
        println!("{} bitcons have been spent", self.0)
    }
}

pub struct EtheriumCoin(u32);

impl EtheriumCoin {
    fn new(amount: u32) -> Self {
        EtheriumCoin(amount)
    }
}

impl Coin for EtheriumCoin {
    fn spend(self: Box<Self>) {
        println!("{} etherium have been spent", self.0)
    }
}

pub trait Wallet {
    fn get(&mut self, amount: u32) -> Option<Box<Coin>>;
}

pub struct BitcoinWallet(u32);

impl BitcoinWallet {
    pub fn new(amount: u32) -> Self {
        BitcoinWallet(amount)
    }
}

impl Wallet for BitcoinWallet {
    fn get(&mut self, amount: u32) -> Option<Box<Coin>> {
        if self.0 < amount {
            println!(
                "Bitcoin wallet doesn't have enough money. Has {}, requested {}",
                self.0,
                amount
            );
            None
        } else {
            self.0 -= amount;
            Some(Box::new(BitcoinCoin::new(amount)))
        }
    }
}

pub struct EtheriumWallet(u32);

impl EtheriumWallet {
    pub fn new(amount: u32) -> Self {
        EtheriumWallet(amount)
    }
}

impl Wallet for EtheriumWallet {
    fn get(&mut self, amount: u32) -> Option<Box<Coin>> {
        if self.0 < amount {
            println!(
                "Etherium wallet doesn't have enough money. Has {}, requested {}",
                self.0,
                amount
            );
            None
        } else {
            self.0 -= amount;
            Some(Box::new(EtheriumCoin::new(amount)))
        }
    }
}

pub fn main() {
    let wallet_vec: Vec<Box<Wallet>> = vec![
        Box::new(BitcoinWallet::new(42)),
        Box::new(EtheriumWallet::new(42)),
    ];

    for mut wallet in wallet_vec {
        wallet.get(30).map_or((), Coin::spend);
        wallet.get(15).map_or((), Coin::spend);
    }
}

use std::option::*;

pub struct USD(f32);

pub trait Coin {
    fn spend(self: Box<Self>);
    fn amount(&self) -> f32;
}

#[derive(Debug)]
pub struct BitcoinCoin(f32);

impl BitcoinCoin {
    pub fn new(amount: f32) -> Self {
        BitcoinCoin(amount)
    }
}

impl Coin for BitcoinCoin {
    fn spend(self: Box<Self>) {
        println!("{} bitcons have been spent", self.0)
    }
    fn amount(&self) -> f32 {
        self.0
    }
}

#[derive(Debug)]
pub struct EtheriumCoin(f32);

impl EtheriumCoin {
    pub fn new(amount: f32) -> Self {
        EtheriumCoin(amount)
    }
}

impl Coin for EtheriumCoin {
    fn spend(self: Box<Self>) {
        println!("{} etherium have been spent", self.0)
    }
    fn amount(&self) -> f32 {
        self.0
    }
}

pub trait Wallet {
    fn get(&mut self, amount: USD) -> Option<Box<Coin>>;
}

pub struct BitcoinWallet(f32);

impl BitcoinWallet {
    pub fn new(amount: f32) -> Self {
        BitcoinWallet(amount)
    }
}

static BITCOIN_RATE: f32 = 6468.37;

impl Wallet for BitcoinWallet {
    fn get(&mut self, amount: USD) -> Option<Box<Coin>> {
        let btc_amount = amount.0 / BITCOIN_RATE;

        if self.0 < btc_amount {
            println!(
                "Bitcoin wallet doesn't have enough money. Has {}, requested {}",
                self.0,
                btc_amount
            );
            None
        } else {
            self.0 -= btc_amount;
            Some(Box::new(BitcoinCoin::new(btc_amount)))
        }
    }
}

pub struct EtheriumWallet(f32);

impl EtheriumWallet {
    pub fn new(amount: f32) -> Self {
        EtheriumWallet(amount)
    }
}

static ETHERIUM_RATE: f32 = 318.04;

impl Wallet for EtheriumWallet {
    fn get(&mut self, amount: USD) -> Option<Box<Coin>> {
        let eth_amount = amount.0 / ETHERIUM_RATE;

        if self.0 < eth_amount {
            println!(
                "Etherium wallet doesn't have enough money. Has {}, requested {}",
                self.0,
                eth_amount
            );
            None
        } else {
            self.0 -= eth_amount;
            Some(Box::new(EtheriumCoin::new(eth_amount)))
        }
    }
}

pub fn main() {
    let wallet_vec: Vec<Box<Wallet>> = vec![
        Box::new(BitcoinWallet::new(0.02)),
        Box::new(EtheriumWallet::new(0.4)),
    ];

    for mut wallet in wallet_vec {
        wallet.get(USD(100.)).map_or((), Coin::spend);
        wallet.get(USD(100.)).map_or((), Coin::spend);
    }
}

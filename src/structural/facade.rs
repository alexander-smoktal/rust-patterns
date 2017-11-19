#[derive(Debug)]
pub struct USDWallet(f32);

#[derive(Debug)]
pub struct BitcoinWallet(f32);

#[derive(Debug)]
pub struct EtheriumWallet(f32);

#[derive(Debug)]
pub struct Wallets {
    usd_wallet: USDWallet,
    btc_wallet: BitcoinWallet,
    eth_wallet: EtheriumWallet,
}

impl Wallets {
    pub fn new() -> Self {
        Wallets {
            usd_wallet: USDWallet(0f32),
            btc_wallet: BitcoinWallet(0f32),
            eth_wallet: EtheriumWallet(0f32),
        }
    }
}

pub fn main() {
    let wallets = Wallets::new();

    println!("Wallets: {:?}", wallets);
}

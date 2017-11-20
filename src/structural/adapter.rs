pub struct USD(f32);

pub trait Wallet: std::fmt::Debug {
    fn add_money(&mut self, amount: USD);
    fn retrieve_money(&mut self, amount: USD) -> Option<USD>;
}

#[derive(Debug)]
pub struct USDWallet {
    money: f32,
}

impl USDWallet {
    pub fn new() -> Self {
        USDWallet { money: 0f32 }
    }
}

impl Wallet for USDWallet {
    fn add_money(&mut self, amount: USD) {
        self.money += amount.0
    }

    fn retrieve_money(&mut self, amount: USD) -> Option<USD> {
        if amount.0 > self.money {
            None
        } else {
            self.money -= amount.0;
            Some(amount)
        }
    }
}

pub struct Bitcoin(f32);

#[derive(Debug)]
pub struct BitcoinWallet {
    bitcoins: f32,
}

impl BitcoinWallet {
    fn add_bitcoins(&mut self, amount: Bitcoin) {
        self.bitcoins += amount.0;
    }

    fn retrieve_bitcoins(&mut self, amount: Bitcoin) -> Option<Bitcoin> {
        if amount.0 > self.bitcoins {
            None
        } else {
            self.bitcoins -= amount.0;
            Some(amount)
        }
    }
}

#[derive(Debug)]
pub struct BitcoinWalletAdapter {
    btc_wallet: BitcoinWallet,
}

impl BitcoinWalletAdapter {
    pub fn new() -> Self {
        BitcoinWalletAdapter { btc_wallet: BitcoinWallet { bitcoins: 0f32 } }
    }
}

static BITCOIN_RATE: f32 = 6468.37;

impl Wallet for BitcoinWalletAdapter {
    fn add_money(&mut self, amount: USD) {
        let btc_amount = Bitcoin(amount.0 / BITCOIN_RATE);

        self.btc_wallet.add_bitcoins(btc_amount)
    }

    fn retrieve_money(&mut self, amount: USD) -> Option<USD> {
        let btc_amount = Bitcoin(amount.0 / BITCOIN_RATE);

        self.btc_wallet.retrieve_bitcoins(btc_amount).map(
            |_a| amount,
        )
    }
}

pub fn main() {
    let wallet_vec: Vec<Box<Wallet>> = vec![
        Box::new(USDWallet::new()),
        Box::new(BitcoinWalletAdapter::new()),
    ];

    for ref mut wallet in wallet_vec {
        println!("Initial wallet: {:?}", wallet);

        wallet.add_money(USD(100f32));
        println!("Added 100 USD: {:?}", wallet);

        wallet.retrieve_money(USD(42f32));
        println!("Retrieved 42 USD: {:?}", wallet);
    }


}

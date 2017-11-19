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

pub trait MergeCoin: Coin {
    fn merge(self, other: &MergeCoin) -> Self where Self: Sized;
}

impl MergeCoin for BitcoinCoin {
    fn merge(self, other: &MergeCoin) -> Self {
        BitcoinCoin::new(self.amount() + other.amount())
    }
}

impl MergeCoin for EtheriumCoin {
    fn merge(self, other: &MergeCoin) -> Self {
        EtheriumCoin::new(self.amount() + other.amount())
    }
}

pub trait CoinMerger {
    type M: MergeCoin + 'static;

    fn merge_n(&self, n: usize) -> Self::M {
        (0..(n - 1)).fold(self.get_coin(), |acc, _val| acc.merge(&self.get_coin()))
    }

    fn get_coin(&self) -> Self::M;
}

pub struct BitcoinMerger;

impl CoinMerger for BitcoinMerger {
    type M = BitcoinCoin;

    fn get_coin(&self) -> Self::M {
        println!("Spawning a bitcoin");
        BitcoinCoin::new(1f32)
    }
}

pub struct EtheriumMerger;

impl CoinMerger for EtheriumMerger {
    type M = EtheriumCoin;

    fn get_coin(&self) -> Self::M {
        println!("Spawning an ether");
        EtheriumCoin::new(1f32)
    }
}

pub fn main() {
    let btc_merger = BitcoinMerger;
    println!("Merged some bitcoins: {:?}", btc_merger.merge_n(5));

    let eth_merger = EtheriumMerger;
    println!("Merged some etherium: {:?}", eth_merger.merge_n(5));
}

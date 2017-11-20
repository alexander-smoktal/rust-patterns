use std::ops::AddAssign;

#[derive(Debug)]
pub struct USD(f32);

impl AddAssign for USD {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

pub trait IntoUSD {
    fn into_usd(self: Box<Self>) -> USD;
}


#[derive(Debug)]
pub enum Cryptocurrency {
    Bitcoin(f32),
    Etherium(f32),
}

static BITCOIN_RATE: f32 = 6468.37;
static ETHERIUM_RATE: f32 = 318.04;

impl IntoUSD for Cryptocurrency {
    fn into_usd(self: Box<Self>) -> USD {
        let self_string = format!("{:?}", self);

        let usd = USD(match *self {
            Cryptocurrency::Bitcoin(amount) => amount * BITCOIN_RATE,
            Cryptocurrency::Etherium(amount) => amount * ETHERIUM_RATE,
        });

        println!("Consuming {} in favor of {:?}", self_string, usd);
        usd
    }
}

pub trait MoneyConsumer {
    fn consume(self) -> USD;
}

pub struct CryptoConsumer {
    currency: Box<IntoUSD>,
}

impl CryptoConsumer {
    pub fn new(currency: Box<IntoUSD>) -> Self {
        CryptoConsumer { currency }
    }
}

impl MoneyConsumer for CryptoConsumer {
    fn consume(self) -> USD {
        self.currency.into_usd()
    }
}

pub fn main() {
    let consumer_vec = vec![
        CryptoConsumer::new(Box::new(Cryptocurrency::Bitcoin(0.2f32))),
        CryptoConsumer::new(Box::new(Cryptocurrency::Etherium(0.42f32))),
    ];
    let mut sum = USD(0f32);

    for consumer in consumer_vec {
        sum += consumer.consume()
    }

    println!("Consumed cryptocurrency for total: {:?}", sum)
}

use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Currencies {
    USD,
    Bitcoin,
    Etherium,
}

#[derive(Debug)]
pub struct Money {
    money: f32,
}

impl Money {
    pub fn store(&mut self, amount: f32) {
        self.money += amount
    }

    pub fn withdraw(&mut self, amount: f32) -> bool {
        if amount <= self.money {
            self.money -= amount;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    currencies: HashMap<Currencies, Arc<RwLock<Money>>>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet { currencies: HashMap::new() }
    }
    pub fn get(&mut self, currency: Currencies) -> Arc<RwLock<Money>> {
        if !self.currencies.contains_key(&currency) {
            self.currencies.insert(
                currency.clone(),
                Arc::new(RwLock::new(Money { money: 0f32 })),
            );
        }

        self.currencies.get(&currency).unwrap().clone()
    }
}

pub fn main() {
    let mut wallet = Wallet::new();

    println!("Initial wallet {:?}", wallet);

    {
        let usd_arc = wallet.get(Currencies::USD);
        usd_arc.write().unwrap().store(100f32);

        let btc_arc = wallet.get(Currencies::Bitcoin);
        btc_arc.write().unwrap().store(100f32);
    }

    println!("Wallet after store {:?}", wallet);

    {
        let usd_arc = wallet.get(Currencies::USD);
        usd_arc.write().unwrap().withdraw(1f32);

        let btc_arc = wallet.get(Currencies::Bitcoin);
        btc_arc.write().unwrap().withdraw(42f32);

        let eth_arc = wallet.get(Currencies::Etherium);
        eth_arc.write().unwrap().store(1f32);
    }

    println!("Wallet at the end {:?}", wallet);

}

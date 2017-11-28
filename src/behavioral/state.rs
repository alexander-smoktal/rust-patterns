pub trait MoneyHandler {
    fn store(&mut self, amount: f32);
}

#[derive(Debug)]
pub struct Wallet(f32);

impl MoneyHandler for Wallet {
    fn store(&mut self, amount: f32) {
        self.0 += amount;
        println!("Wallet received money: {:?}", self);
    }
}

pub struct WalletBalancer {
    wallets: Vec<Box<MoneyHandler>>,
    index: usize,
}

impl WalletBalancer {
    pub fn new(wallets: Vec<Box<MoneyHandler>>) -> WalletBalancer {
        WalletBalancer { wallets, index: 0 }
    }

    pub fn add_money(&mut self, amount: f32) {
        self.wallets.get_mut(self.index).unwrap().store(amount);

        self.index = if self.index == self.wallets.len() - 1 {
            0
        } else {
            self.index + 1
        }
    }
}

pub fn main() {
    let mut balancer = WalletBalancer::new(vec![
        Box::new(Wallet(0f32)),
        Box::new(Wallet(0f32)),
        Box::new(Wallet(0f32)),
    ]);

    balancer.add_money(1f32);
    balancer.add_money(2f32);
    balancer.add_money(3f32);
    balancer.add_money(10f32);
    balancer.add_money(20f32);
    balancer.add_money(30f32);
}

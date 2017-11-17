use std::result::Result;

pub trait Wallet {
    fn store(&mut self, amount: f32);
    fn withdraw(&mut self, amount: f32) -> Result<(), &str>;
}

#[derive(Debug)]
pub struct USDWallet(f32);

impl Wallet for USDWallet {
    fn store(&mut self, amount: f32) {
        self.0 += amount
    }

    fn withdraw(&mut self, amount: f32) -> Result<(), &str> {
        if self.0 >= amount {
            self.0 -= amount;
            Ok(())
        } else {
            Err("Not enough money")
        }
    }
}

#[derive(Debug)]
pub struct Debts {
    wallet: USDWallet,
    debt: f32
}

impl Debts {
    pub fn new() -> Self {
        Debts {
            wallet: USDWallet(0f32),
            debt: 0f32
        }
    }
}

impl Wallet for Debts {
    fn store(&mut self, mut amount: f32) {
        if self.debt > 0f32 {
            self.debt -= amount
        }

        if self.debt < 0f32 {
            amount = -self.debt;
            self.debt = 0f32
        }

        self.wallet.store(amount);
    }

    fn withdraw(&mut self, mut amount: f32) -> Result<(), &str> {
        if self.debt > 0f32 {
            Err("Can't withdraw. You have a debt")
        } else {
            let new_debt = self.wallet.0 - amount;

            if new_debt < 0f32 {
                self.debt = -new_debt;
                amount += new_debt;
            }

            self.wallet.withdraw(amount)
        }
    }
}

pub fn main() {
    let mut debt_wallet = Debts::new();
    println!("Initial wallet: {:?}", debt_wallet);

    print!("Storing 100: {:?}. ", debt_wallet.store(100f32));
    println!("Wallet {:?}", debt_wallet);
    print!("Withdrawing 42: {:?}. ", debt_wallet.withdraw(42f32));
    println!("Wallet {:?}", debt_wallet);
    print!("Withdrawing 82: {:?}. ", debt_wallet.withdraw(82f32));
    println!("Wallet {:?}", debt_wallet);
    print!("Withdrawing 5: {:?}. ", debt_wallet.withdraw(5f32));
    println!("Wallet {:?}", debt_wallet);
    print!("Storing 35: {:?}. ", debt_wallet.store(35f32));
    println!("Wallet {:?}", debt_wallet);
}
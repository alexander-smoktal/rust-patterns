use std::fmt;

pub trait SpendStrategy {
    fn spend(&self);
}

pub type SpendStrategyFunction = fn(&mut SpendStrategy, f32);

pub struct Money(f32, fn(&Money));

impl Money {
    fn send_strategy_function(&self) {
        println!("Sending some money: {:?}", self)
    }

    fn burn_strategy_function(&self) {

        println!("Burning some money: {:?}", self)
    }
}

impl SpendStrategy for Money {
    fn spend(&self) {
        self.1(self)
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Money({})", self.0)
    }
}

pub fn main() {
    let send_money = Money(100f32, Money::send_strategy_function);
    let burn_money = Money(100f32, Money::burn_strategy_function);

    send_money.spend();
    burn_money.spend();
}

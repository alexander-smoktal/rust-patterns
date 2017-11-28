use std::rc::Rc;
use std::cell::RefCell;

#[derive(Copy, Clone)]
pub enum State {
    Added,
    Spent
}

pub trait Observer {
    fn notify(&self, state: State, amount: f32);
}

pub trait Observable {
    fn register_observer(&mut self, observer: Rc<RefCell<Box<Observer>>>);
    fn notify_observables(&self, state: State, amount: f32);
}

pub struct USDObserver;

impl Observer for USDObserver {
    fn notify(&self, state: State, amount: f32) {
        match state {
            State::Added => println!("Added {} USD", amount),
            _ => println!("Spent {} USD", amount)
        }
    }
}

pub struct BitcoinObserver;

const BITCOIN_RATE: f32 = 6468.37;

impl Observer for BitcoinObserver {
    fn notify(&self, state: State, amount: f32) {
        match state {
            State::Added => println!("Added {} BTC", amount / BITCOIN_RATE),
            _ => println!("Spent {} BTC", amount / BITCOIN_RATE)
        }
    }
}

pub struct EtheriumObserver;

const ETHERIUM_RATE: f32 = 318.04;

impl Observer for EtheriumObserver {
    fn notify(&self, state: State, amount: f32) {
        match state {
            State::Added => println!("Added {} ETH", amount / ETHERIUM_RATE),
            _ => println!("Spent {} ETH", amount / ETHERIUM_RATE)
        }
    }
}

struct Wallet {
    observables: Vec<Rc<RefCell<Box<Observer>>>>
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            observables: vec![]
        }
    }
}

impl Observable for Wallet {
    fn register_observer(&mut self, observer: Rc<RefCell<Box<Observer>>>) {
        self.observables.push(observer)
    }

    fn notify_observables(&self, state: State, amount: f32) {
        for observer in self.observables.iter() {
            observer.borrow().notify(state, amount)
        }
    }
}

pub fn main() {
    let mut wallet = Wallet::new();

    {
        let usd_observer = Rc::new(RefCell::new(Box::new(USDObserver) as Box<Observer>));
        wallet.register_observer(usd_observer);

        let btc_observer = Rc::new(RefCell::new(Box::new(BitcoinObserver) as Box<Observer>));
        wallet.register_observer(btc_observer);

        let eth_observer = Rc::new(RefCell::new(Box::new(EtheriumObserver) as Box<Observer>));
        wallet.register_observer(eth_observer);
    }

    wallet.notify_observables(State::Added, 100f32);
    wallet.notify_observables(State::Spent, 42f32);
}


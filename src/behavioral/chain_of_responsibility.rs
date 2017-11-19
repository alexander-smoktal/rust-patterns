#[derive(Debug, Clone)]
pub enum Currency {
    Bitcoin(f32),
    Etherium(f32),
    USD(f32),
    Invalid
}

#[derive(Debug)]
pub struct NotFound;

pub trait CurrencyPrinter {
    fn new() -> Self where Self: Sized;
    fn print(&self, currency: Currency) -> Result<(), NotFound>;
}

pub struct PrinterWrapper<T> where T: CurrencyPrinter {
    this: T,
    next: Option<Box<CurrencyPrinter>>
}

impl<T> PrinterWrapper<T> where T: CurrencyPrinter {
    pub fn new(next: Option<Box<CurrencyPrinter>>) -> Self {
        PrinterWrapper::<T> {
            this: T::new(),
            next
        }
    }
}

impl<T> CurrencyPrinter for PrinterWrapper<T> where T: CurrencyPrinter {
    fn new() -> Self { 
        PrinterWrapper::<T> {
            this: T::new(),
            next: None
        } 
    }

    fn print(&self, currency: Currency) -> Result<(), NotFound> {
        match self.this.print(currency.clone()) {
            Ok(_) => Ok(()),
            Err(_) => {
                match self.next {
                    Some(ref printer) => printer.print(currency),
                    _ => Err(NotFound)
                }
            }
        }
    }
}

pub struct BitcoinPrinter;
type BTCPrinter = PrinterWrapper<BitcoinPrinter>;

impl CurrencyPrinter for BitcoinPrinter {
    fn new() -> Self { BitcoinPrinter }
    fn print(&self, currency: Currency) -> Result<(), NotFound> {
        match currency {
            Currency::Bitcoin(amount) => Ok(println!("BTC: {}", amount)),
            _ => Err(NotFound)
        }
        
    }
}

pub struct EtheriumPrinter;
type ETHPrinter = PrinterWrapper<EtheriumPrinter>;

impl CurrencyPrinter for EtheriumPrinter {
    fn new() -> Self { EtheriumPrinter }
    fn print(&self, currency: Currency) -> Result<(), NotFound> {
        match currency {
            Currency::Etherium(amount) => Ok(println!("ETH: {:?}", amount)),
            _ => Err(NotFound)
        }
        
    }
}

pub struct USDPrinter;
type USPrinter = PrinterWrapper<USDPrinter>;

impl CurrencyPrinter for USDPrinter {
    fn new() -> Self { USDPrinter }
    fn print(&self, currency: Currency) -> Result<(), NotFound> {
        match currency {
            Currency::USD(amount) => Ok(println!("USD: {:?}", amount)),
            _ => Err(NotFound)
        }
        
    }
}

pub fn main() {
    let chain = BTCPrinter::new(
        Some(Box::new(ETHPrinter::new(
            Some(Box::new(USPrinter::new(None)))))));

    println!("Printing BTC: {:?}", chain.print(Currency::Bitcoin(42f32)));
    println!("Printing ETH: {:?}", chain.print(Currency::Etherium(100f32)));
    println!("Printing USD: {:?}", chain.print(Currency::USD(1f32)));
    println!("Printing invalid: {:?}", chain.print(Currency::Invalid));
}
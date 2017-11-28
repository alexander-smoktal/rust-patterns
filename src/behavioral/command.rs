
use std::rc::Rc;
use std::sync::RwLock;

pub trait Command {
    fn execute(&mut self);
}

#[derive(Eq, PartialEq)]
pub enum LogState {
    On,
    Off,
}

pub struct Logger {
    state: LogState,
}

impl Logger {
    pub fn new() -> Self {
        Logger { state: LogState::On }
    }

    fn switch(&mut self, state: LogState) {
        self.state = state
    }

    pub fn log(&self, message: String) {
        if self.state == LogState::On {
            println!("LOG: {}", message)
        }
    }
}

pub struct LogOnCommand {
    logger: Rc<RwLock<Logger>>,
}

impl Command for LogOnCommand {
    fn execute(&mut self) {
        self.logger.write().unwrap().switch(LogState::On);
    }
}

pub struct LogOffCommand {
    logger: Rc<RwLock<Logger>>,
}

impl Command for LogOffCommand {
    fn execute(&mut self) {
        self.logger.write().unwrap().switch(LogState::Off);
    }
}

pub fn main() {
    let logger = Rc::new(RwLock::new(Logger::new()));

    let mut log_on = LogOnCommand { logger: logger.clone() };
    let mut log_off = LogOffCommand { logger: logger.clone() };

    logger.write().unwrap().log("Initial log".into());

    log_off.execute();

    logger.write().unwrap().log("Log disabled".into());

    log_on.execute();

    logger.write().unwrap().log("Log enabled".into());
}

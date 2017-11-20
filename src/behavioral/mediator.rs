use std::collections::HashMap;
use std::sync::RwLock;
use std::rc::Rc;

pub struct Block;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum EventType {
    BlockCreated,
    BlockUpdated,
    BlockDestroyed
}

pub trait EventListener {}

pub type EventHandler = fn(&mut Box<EventListener>, &Block);

pub struct Mediator {
    listeners: HashMap<EventType, Vec<(Rc<RwLock<Box<EventListener>>>, EventHandler)>>
}

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            listeners: HashMap::new()
        }
    }

    pub fn register(&mut self, event: EventType, listener: Rc<RwLock<Box<EventListener>>>, callback: EventHandler) {
        if !self.listeners.contains_key(&event) {
            self.listeners.insert(event.clone(), vec![]);
        }

        let event_callbacks = self.listeners.get_mut(&event).unwrap();
        event_callbacks.push((listener, callback))

    }

    pub fn emit(&mut self, event: EventType, block: &Block) {
        if self.listeners.contains_key(&event) {
            for &mut (ref mut listener, ref callback)
                in self.listeners.get_mut(&event).unwrap().iter_mut() {
                let mut guard = listener.write().unwrap();

                callback(&mut *guard, block)
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BlockLifetimeListener;

impl EventListener for BlockLifetimeListener {
}

impl BlockLifetimeListener {
    pub fn block_created(_this: &mut Box<EventListener>, _block: &Block) {
        println!("Block created");
    }

    pub fn block_destroyed(_this: &mut Box<EventListener>, _block: &Block) {
        println!("Block destroyed");
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct BlockUpdateListener;

impl EventListener for BlockUpdateListener {
}

impl BlockUpdateListener {
    pub fn block_updated(_this: &mut Box<EventListener>, _block: &Block) {
        println!("Block updated");
    }
}

pub fn main() {
    let blifetime = Rc::new(RwLock::new(Box::new(BlockLifetimeListener {}) as Box<EventListener>));
    let bupdate = Rc::new(RwLock::new(Box::new(BlockUpdateListener {}) as Box<EventListener>));

    let mut mediator = Mediator::new();

    mediator.register(EventType::BlockCreated, blifetime.clone(), BlockLifetimeListener::block_created);
    mediator.register(EventType::BlockDestroyed, blifetime.clone(), BlockLifetimeListener::block_destroyed);

    mediator.register(EventType::BlockUpdated, bupdate.clone(), BlockUpdateListener::block_updated);

    let block = Block;

    mediator.emit(EventType::BlockCreated, &block);
    mediator.emit(EventType::BlockUpdated, &block);
    mediator.emit(EventType::BlockDestroyed, &block);
}
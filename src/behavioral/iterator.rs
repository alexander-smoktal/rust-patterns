use std::iter::Iterator;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Transaction(f32, u64);

pub struct Block {
    txs: Rc<Vec<Transaction>>,
}

impl Block {
    pub fn new(txs: Vec<Transaction>) -> Self {
        Block { txs: Rc::new(txs) }
    }

    pub fn iter(&self) -> TransactionIterator {
        TransactionIterator {
            txs: self.txs.clone(),
            index: 0,
        }
    }
}

pub struct TransactionIterator {
    txs: Rc<Vec<Transaction>>,
    index: usize,
}

impl<'iter> Iterator for TransactionIterator {
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.txs.len() {
            let result = Some(self.txs[self.index].clone());
            self.index += 1;
            result
        } else {
            None
        }
    }
}

pub fn main() {
    let block = Block::new(vec![
        Transaction(0f32, 0x12345678),
        Transaction(42f32, 0x12345677),
        Transaction(100f32, 0x12345679),
    ]);

    for ref transaction in block.iter() {
        println!("Iterating: {:?}", transaction)
    }
}

use std::collections::HashMap;

#[derive(Clone)]
pub struct Transaction(f32, u64);

#[derive(Clone)]
pub struct Block {
    incoming_txs: Vec<Transaction>,
    outgoing_txs: Vec<Transaction>
}

pub trait UTXOTrait {
    fn handle_block(&mut self, block: Block) -> Option<String>;
}

#[derive(Debug)]
pub struct UTXO {
    unspended: HashMap<u64, f32>
}

impl UTXO {
    pub fn new() -> Self {
        UTXO {
            unspended: HashMap::new()
        }
    }

    pub fn add_unspended(mut self, account: u64, amount: f32) -> Self {
        self.unspended.insert(account, amount);
        self
    }
}

impl UTXOTrait for UTXO {
    fn handle_block(&mut self, block: Block) -> Option<String> {
        for itx in block.incoming_txs {
            if self.unspended.contains_key(&itx.1) {
                if self.unspended.get(&itx.1) == Some(&itx.0) {
                    self.unspended.remove(&itx.1);
                } else {
                    return Some("Invalid money amount".into())
                }
            } else {
                return Some("UTXO doesn't contain incoming transaction".into());
            }
        }

        for otx in block.outgoing_txs {
            if self.unspended.contains_key(&otx.1) {
                return Some("UTXO already contains outgoing transaction".into());
            } else {
                self.unspended.insert(otx.1, otx.0);
            }
        }

        None
    }
}

pub struct SafeUTXO<'utxolt> {
    utxo: &'utxolt UTXO
}

impl<'utxolt> SafeUTXO<'utxolt> {
    pub fn new(utxo: &'utxolt UTXO) -> Self {
        SafeUTXO {
            utxo
        }
    }
}

impl<'utxolt> UTXOTrait for SafeUTXO<'utxolt> {
    fn handle_block(&mut self, block: Block) -> Option<String> {
        let unspended_copy = self.utxo.unspended.clone();

        UTXO { unspended: unspended_copy }.handle_block(block)
    }
}

pub fn main() {
    let mut source_utxo = UTXO::new()
        .add_unspended(1, 1.0)
        .add_unspended(2, 2.0);

    println!("Start utxo {:?}", source_utxo);

    let bad_block = Block {
        incoming_txs: vec![Transaction(1.0, 3)],
        outgoing_txs: vec![Transaction(2.0, 1)]
    };

    let good_block = Block {
        incoming_txs: vec![Transaction(1.0, 1)],
        outgoing_txs: vec![Transaction(3.0, 3)]
    };

    {
        let mut safe_utxo = SafeUTXO::new(&source_utxo);

        println!("Checking bad block: {:?}", safe_utxo.handle_block(bad_block.clone()));
        println!("Checking good block: {:?}", safe_utxo.handle_block(good_block.clone()))
    }

    println!("Start utxo after safe check: {:?}", source_utxo);

    source_utxo.handle_block(good_block);

    println!("Start utxo after actual handling good block: {:?}", source_utxo);
}

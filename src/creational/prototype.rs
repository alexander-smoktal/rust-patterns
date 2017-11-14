#[derive(Debug, Clone)]
pub struct Transaction {
    amount: f32,
    address: u64,
}

impl Transaction {
    pub fn new(amount: f32, address: u64) -> Self {
        Transaction { amount, address }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    hash: u64,
    incoming_txs: Vec<Transaction>,
    outgoing_txs: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            hash: 0u64,
            incoming_txs: vec![],
            outgoing_txs: vec![],
        }
    }

    fn recalc_hash(mut self) -> Self {
        self.hash = self.incoming_txs
            .iter()
            .chain(self.outgoing_txs.iter())
            .fold(0u64, |acc, ref tx| acc + tx.address);

        self
    }

    pub fn add_incoming_transaction(mut self, tx: Transaction) -> Self {
        self.incoming_txs.push(tx);
        self.recalc_hash()
    }

    pub fn add_outgoing_transaction(mut self, tx: Transaction) -> Self {
        self.outgoing_txs.push(tx);
        self.recalc_hash()
    }
}

pub fn main() {
    let start_block = Block::new()
        .add_incoming_transaction(Transaction::new(5f32, 0x12345678))
        .add_outgoing_transaction(Transaction::new(10f32, 0x12345678));

    let cloned_block = start_block.clone().add_incoming_transaction(
        Transaction::new(42f32, 0x12345679),
    );

    println!(
        "Source block: {:?}\nCloned block: {:?}",
        start_block,
        cloned_block
    );
}

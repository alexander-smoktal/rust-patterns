#[derive(Debug)]
pub struct Transaction {
    amount: f32,
    address: u64,
}

impl Transaction {
    pub fn new(amount: f32, address: u64) -> Self {
        Transaction { amount, address }
    }
}

#[derive(Debug)]
pub struct Block {
    hash: u64,
    incoming_txs: Vec<Transaction>,
    outgoing_txs: Vec<Transaction>,
}

pub trait BlockBuilder {
    fn add_incoming_transaction(self, tx: Transaction) -> Self;
    fn add_outgoing_transaction(self, tx: Transaction) -> Self;
    fn finalize(self) -> Block;
}

pub struct GenericBlockBuilder {
    incoming_txs: Vec<Transaction>,
    outgoing_txs: Vec<Transaction>,
}

impl GenericBlockBuilder {
    pub fn new() -> Self {
        GenericBlockBuilder {
            incoming_txs: vec![],
            outgoing_txs: vec![],
        }
    }
}

impl BlockBuilder for GenericBlockBuilder {
    fn add_incoming_transaction(mut self, tx: Transaction) -> Self {
        self.incoming_txs.push(tx);
        self
    }

    fn add_outgoing_transaction(mut self, tx: Transaction) -> Self {
        self.outgoing_txs.push(tx);
        self
    }

    fn finalize(self) -> Block {
        let hash = self.incoming_txs.iter().chain(self.outgoing_txs.iter())
            .fold(0u64,
                  |acc, ref tx|
                      acc + tx.address
            );

        let GenericBlockBuilder {
            incoming_txs,
            outgoing_txs,
        } = self;

        Block {
            hash,
            incoming_txs,
            outgoing_txs,
        }
    }
}

pub struct GenesisBlockBuilder {
    genesis_tx: Transaction,
}

impl GenesisBlockBuilder {
    pub fn new(genesis_tx: Transaction) -> Self {
        GenesisBlockBuilder { genesis_tx }
    }
}

impl BlockBuilder for GenesisBlockBuilder {
    fn add_incoming_transaction(self, _tx: Transaction) -> Self {
        println!("Failed to add transaction into genesis block");
        self
    }

    fn add_outgoing_transaction(self, tx: Transaction) -> Self {
        self.add_incoming_transaction(tx)
    }

    fn finalize(self) -> Block {
        Block {
            hash: self.genesis_tx.address,
            incoming_txs: vec![self.genesis_tx],
            outgoing_txs: vec![],
        }
    }
}

pub fn main() {
    let genesis = GenesisBlockBuilder::new(Transaction::new(42f32, 0x12345678))
        .add_incoming_transaction(Transaction::new(10f32, 0x12345678))
        .add_incoming_transaction(Transaction::new(20f32, 0x12345678))
        .finalize();
    println!("Creted genesis block {:?}", genesis);

    let generic = GenericBlockBuilder::new()
        .add_incoming_transaction(Transaction::new(10f32, 0x12345678))
        .add_incoming_transaction(Transaction::new(20f32, 0x12345678))
        .add_outgoing_transaction(Transaction::new(30f32, 0x12345679))
        .finalize();
    println!("Created generic block {:?}", generic);
}

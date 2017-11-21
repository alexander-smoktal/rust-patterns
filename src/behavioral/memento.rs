#[derive(Debug)]
pub struct Block(u64);

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Block(std::u64::MAX)]
        }
    }

    pub fn push_block(&mut self, block: Block) -> BlockChainState {
        let result = BlockChainState(self.blocks.last().unwrap().0);

        self.blocks.push(block);
        result
    }

    pub fn restore_state(&mut self, state: BlockChainState) -> Result<(), &str> {
        while self.blocks.len() > 0 {
            if self.blocks.last().unwrap().0 == state.0 {
                return Ok(());
            } else {
                self.blocks.pop();
            }
        }

        Err("State not found")
    }
}

pub struct BlockChainState(u64);

pub fn main() {
    let mut blockchain = BlockChain::new();
    blockchain.push_block(Block(0x0));
    println!("Start blockchain {:?}", blockchain);

    let state = blockchain.push_block(Block(0x1));
    blockchain.push_block(Block(0x2));
    println!("3-block blockchain: {:?}", blockchain);

    println!("Restoring state: {:?}", blockchain.restore_state(state));
    println!("Restored blockchain: {:?}", blockchain)
}

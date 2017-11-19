use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug)]
pub struct Hash(u64);

impl Add for Hash {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Hash(self.0 + other.0)
    }
}

#[derive(Debug, Clone)]
pub enum MerkleTree {
    Leaf(Hash),
    Tree(Hash, Box<MerkleTree>, Box<MerkleTree>),
}

impl MerkleTree {
    pub fn new(hash: Hash) -> Self {
        MerkleTree::Leaf(hash)
    }

    fn get_hash(&self) -> Hash {
        match self {
            &MerkleTree::Leaf(oh) => oh,
            &MerkleTree::Tree(oh, _, _) => oh,
        }
    }
}

impl AddAssign<Hash> for MerkleTree {
    fn add_assign(&mut self, hash: Hash) {
        *self = MerkleTree::Tree(hash + self.get_hash(),
                                 Box::new(MerkleTree::Leaf(hash)),
                                 Box::new(self.clone()))
    }
}

impl AddAssign for MerkleTree {
    fn add_assign(&mut self, tree: MerkleTree) {
        *self = MerkleTree::Tree(tree.get_hash() + self.get_hash(),
                                 Box::new(tree),
                                 Box::new(self.clone()))
    }
}

pub fn main() {
    let mut tree1 = MerkleTree::new(Hash(0x12345678));
    let mut tree2 = MerkleTree::new(Hash(0x12345123));
    println!("Initial trees:\n{:?}\n{:?}", tree1, tree2);

    tree1 += Hash(0x12345679);
    tree2 += Hash(0x12345123);
    println!("Additional hash trees:\n{:?}\n{:?}", tree1, tree2);

    tree1 += tree2;
    println!("Resulting tree:\n{:?}", tree1);
}

use chrono::Utc;
use sha256::digest;

use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchan {
    pub blocks: Vec<Block>,
}

impl Blockchan {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn starting_block(a: &mut Self) {
        let genesis_block = Block {
            id: 1,
            nounce: 11111,
            data: String::from("I am a first genesis block"),
            hash: String::from("0000000000000000000000000000000000000000000000000000000000558090"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            ),
            timpstamp: Utc::now().timestamp(),
        };
        a.blocks.push(genesis_block);
    }

    pub fn try_add_new_block(b: &mut Self, block: Block) {
        match b.blocks.last() {
            None => {
                println!("the blockchain does not have at least one block");
                return;
            }
            Some(latest_block) => {
                if b.is_block_valid(&block, latest_block) {
                    b.blocks.push(block);
                    println!("Block has been added Successfully");
                } else {
                    println!("Block is Invalid!");
                }
            }
        }
    }

    pub fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Block with id {} has wrong previous hash", new_block.id);
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("Block with id {} has wrong hash", new_block.id);
            return false;
        } else if new_block.id != latest_block.id + 1 {
            println!(
                "Block with id {} is not the next block after the latest block with id {}",
                new_block.id, latest_block.id
            );
            return false;
        } else if digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            new_block.timpstamp,
            new_block.previous_hash,
            new_block.data,
            new_block.nounce
        )) != new_block.hash
        {
            println!("Block with id {} has wrong hash", new_block.id);
            return false;
        }
        true
    }

    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!(
                "Block ID: {}\nNonce: {}\nData: {}\nHash: {}\nPrevious Hash: {}\nTimestamp: {}\n",
                block.id,
                block.nounce,
                block.data,
                block.hash,
                block.previous_hash,
                block.timpstamp
            );
        }
    }

    pub fn is_chain_valid(s: &Self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("The Chain is Empty"),
            1 => println!("The chain only contains one single block"),
            _ => {
                for i in 1..chain.len() {
                    let prev = &chain[i - 1];
                    let current = &chain[i];
                    if !s.is_block_valid(current, prev) {
                        return false;
                    }
                }
            }
        }
        println!("The Chain is found to be correct and valid");
        true
    }
}

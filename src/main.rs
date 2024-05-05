use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)]
pub struct Blockchan {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub nounce: u64,
    pub data: String,
    pub hash: String,
    pub previous_hash: String,
    pub timpstamp: i64,
}

impl Blockchan {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn starting_block(a: &mut Self) {
        let genisis_block = Block {
            id: 1,
            nounce: 11111,
            data: String::from("I am a first genesis block"),
            hash: String::from("0000000000000000000000000000000000000000000000000000000000558090"),
            previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            timpstamp: Utc::now().timestamp(),
        };
        a.blocks.push(genisis_block);
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
        if new_block.hash == latest_block.previous_hash {
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
            new_block.id, new_block.timpstamp, new_block.previous_hash, new_block.data, new_block.nounce
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
                "Block ID: {}\nNounce: {}\nData: {}\nHash: {}\nPrevious Hash: {}\nTimestamp: {}\n",
                block.id, block.nounce, block.data, block.hash, block.previous_hash, block.timpstamp
            );
        }
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let now_timestamp = now.timestamp();

        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);

        Self {
            id: id,
            nounce: nonce,
            data: data,
            hash: hash,
            previous_hash: previous_hash,
            timpstamp: now.timestamp(),
        }
    }

    pub fn mine_block(id: u64, time_stamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("previous hash: {}", previous_hash);
        let mut nonce = 1;
        loop {
            let block_string = format!("{}{}{}{}{}", id, time_stamp, previous_hash, data, nonce);
            let hash = digest(block_string);
            if hash.starts_with("0000") {
                println!("mined! nonce {}, hash {}", nonce, hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut bc = Blockchan::new();
    Blockchan::starting_block(&mut bc);

    println!("Initial Blockchain:");
    bc.print_blocks();

    // Get the previous block hash
    let previous_hash = bc.blocks.last().unwrap().hash.clone();
    // Create the new block with the correct previous hash
    let new_block = Block::new(2, previous_hash, "Helloooo".to_string());
    Blockchan::try_add_new_block(&mut bc, new_block);
    println!("Updated Blockchain:");
    bc.print_blocks();

    let previous_hash1 = bc.blocks.last().unwrap().hash.clone();
    let new_block1 = Block::new(3, previous_hash1, "Helloooo".to_string());
    Blockchan::try_add_new_block(&mut bc, new_block1);
    println!("Updated Blockchain:");
    bc.print_blocks();
}

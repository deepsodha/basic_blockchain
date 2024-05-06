use crate::{block::Block, blockchain::Blockchan};

mod block;
mod blockchain;

fn main() {
    let mut bc = Blockchan::new();
    Blockchan::starting_block(&mut bc);

    println!("Initial Blockchain:");
    bc.print_blocks();
    let previous_hash = bc.blocks.last().unwrap().hash.clone();
    let new_block = Block::new(2, previous_hash, "Helloooo".to_string());
    Blockchan::try_add_new_block(&mut bc, new_block);
    println!("Updated Blockchain:");
    bc.print_blocks();

    let previous_hash1 = bc.blocks.last().unwrap().hash.clone();
    let new_block1 = Block::new(3, previous_hash1, "Helloooo".to_string());
    Blockchan::try_add_new_block(&mut bc, new_block1);
    println!("Updated Blockchain:");
    bc.print_blocks();

    Blockchan::is_chain_valid(&bc, &bc.blocks);
}

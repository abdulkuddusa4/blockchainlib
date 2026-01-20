use blockchainlib::*;
// use blockchainlib::Hashable;
use std::time;
use std::str::FromStr;

fn print_type_of<T>(obj:&T){
    println!("{:?}", std::any::type_name::<T>());
}
union mcv{
    x: i32,
    y: [u8;4]
}

use serde_json::Value;
fn main() {

    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,now(),vec![0;32],0,
        "Genesis block!".to_string(),
        difficulty

    );
    println!("started mining");
    block.mine();
    println!("Mined genesis block {:?}", &block);
    
    let mut blockchain = Blockchain{
        blocks: vec![block]
    };

    for i in 1..=5{
        let mut block = Block::new(
            i, now(), blockchain.blocks[i as usize -1].hash(),0,
            "Another Block".to_string(), difficulty
        );
        block.mine();
        println!("another block {:?}", &block);
        blockchain.blocks.push(block);
    }

    // blockchain.blocks[3].payload="None".to_string();
    println!("verified: {}", blockchain.verify());

}

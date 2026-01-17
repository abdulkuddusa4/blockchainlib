use blockchainlib::Block;
use blockchainlib::Hashable;


fn main() {
    let block = Block::new(
        0,0,vec![0;32],0,"genesis b lock".to_string()

    );
    println!("{:?}", block.hash());

}

use crate::Block;
use crate::Hashable;

pub struct Blockchain{
	pub blocks: Vec<Block>
}


impl Blockchain{
	pub fn verify(&self) -> bool {
		let mut prev_hash = vec![0 as u32;32];
		for (i, block) in self.blocks.iter().enumerate(){
			 if block.index != i as u32{
			 	println!("index mismatch");
			 	return false;
			 }
			 if !crate::check_difficulty(
			 	&block.hash(),
			 	block.difficulty
			 ){
			 	println!("difficulty failed");
			 	return false;
			 }

			 if i!=0{
			 	let prev_block = &self.blocks[i-1];
			 	if block.timestamp <= prev_block.timestamp{
			 		println!("time mismatch");
			 		return false;
			 	}
			 	if prev_block.hash != block.prev_block_hash{
			 		println!("hash mismatch");
			 		return false;
			 	}
			 }
			 else{
			 	if block.prev_block_hash != vec![0;32]{
			 		println!("invalid first prev hash");
			 		return false;
			 	}
			 }
		}
		return true;
	}
}
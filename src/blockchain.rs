use crate::Block;
use crate::Hashable;


pub enum BlockValidationErr{
	MismatchesIndex,
	InvalidHash,
	AchronologicalTimestamp,
	MismatchedPreviousHash,
	InvalidGenesisBlockFormat,
	InavlidInput,
	InsufficientInputValue,
	InvalidCoinbaseTransaction,
}
use BlockValidationErr as BVE;

pub struct Blockchain{
	pub blocks: Vec<Block>,
	unspent_outputs: HashSet<Hash>
}


impl Blockchain{
	pub fn update_with_block(
		&mut self, block: Block
	) -> Result<(), BVE> 
	{
		let mut prev_hash = vec![0 as u32;32];
		if block.index != i as u32{
			println!("index mismatch");
			return Err(BVE::MismatchesIndex);
		}
		if !crate::check_difficulty(
			&block.hash(),
			block.difficulty
		){
			return Err(BVE::InvalidHash);
		}
		if i!=0{
			let prev_block = &self.blocks[i-1];
			if block.timestamp <= prev_block.timestamp{
				return Err(BVE::AchronologicalTimestamp);
			}
			if prev_block.hash != block.prev_block_hash{
				return Err(BVE::MismatchedPreviousHash);
			}
		}
		else{
			if block.prev_block_hash != vec![0;32]{
				return Err(BVE::InvalidGenesisBlockFormat);
			}
		}

		if let Some((Coinbase, transactions))=
		block.transactions.split_first(){
			if !coinbase.is_coinbase(){
				return Err(
					BVE::InvalidCoinbaseTransaction
				);
			}
			let mut block_spent = HashSet::new();

			for transaction in transactions{
				let input_hashes = transaction.input_hashes();
				
				if 
					!(&input_hashes - &self.unspent_outputs)
					.is_empty() ||
					(&input_hashes & &block_spent).is_empty()
				{
					return 
				}
			}
		}
		Ok(())
	}
}
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
pub struct Block {
    time: u64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>
}

impl Block {
    fn new(data: &str, prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data: data.as_bytes().to_vec(),
            prev_block_hash,
            hash: Vec::new()
        };

        block.set_hash();
        block
    }

    fn set_hash(&mut self) {

        let time_bytes = self.time.to_be_bytes();

        let to_be_hashed: Vec<&[u8]> = vec![
            &self.prev_block_hash,
            &self.data,
            &time_bytes,
        ];

        let mut header: Vec<u8> = Vec::new();

        for val in &to_be_hashed {
            header.extend(val.iter().clone())
        }

        let mut sha256 = Sha256::new();
        sha256.update(header);
        self.hash = sha256.finalize().to_vec();
    }

    fn new_genesis_block() -> Block {
        Block::new("This is New Genesis Block", Vec::new())
    }
}
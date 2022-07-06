use chrono::prelude::*;
use utils::coder::*;

pub struct BlockHeader {
    pub timestamp: i64,
    pub tx_hash: String,
    pub prev_hash: String,
}

pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let serialize_data = block_serialize(&data);
        let tx_hash = get_hash(&serialize_data[..]);
        let header = BlockHeader {
            timestamp: timestamp,
            tx_hash: tx_hash,
            prev_hash: prev_hash,
        };
        let serialized_header = block_serialize(&header);
        let hash = get_hash(&serialized_header[..]);
        Block {
            header: header,
            hash: hash,
            data: data,
        }
    }
}

// pub struct BlockChain {
//     pub blocks: Vec<Block>,
// }

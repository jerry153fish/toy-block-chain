use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utils::coder::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    pub timestamp: i64,
    pub tx_hash: String,
    pub prev_hash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_works() {
        let block = Block::new("test".to_string(), "0".to_string());
        assert_eq!(block.data, "test");
        assert_eq!(
            block.header.tx_hash,
            "c3e53ee0e3b2655fb8658831847e5685a30e2c5a5ea83f675b97abe7cb1fc599"
        );
    }
}

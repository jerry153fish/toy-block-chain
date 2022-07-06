use crate::block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Blockchain::new_generic_block()],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let block = block::Block::new(data, prev_hash);
        self.blocks.push(block);
    }

    pub fn new_generic_block() -> block::Block {
        let data = "Genesis Block".to_string();
        let prev_hash = "0".to_string();
        block::Block::new(data, prev_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chain_should_works() {
        let chain = Blockchain::new();
        // println!("{:?}", chain);
        assert_eq!(chain.blocks.len(), 1);
        let generic_block = chain.blocks.first().unwrap();
        assert_eq!(generic_block.data, "Genesis Block");
        assert_eq!(generic_block.header.prev_hash, "0");
        assert_eq!(
            generic_block.header.tx_hash,
            "668e5cb7294ae339c468bd1e683438af8da220822a7cef2a323d335ab7088391"
        );
    }
}

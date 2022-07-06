use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn block_serialize<T: ?Sized>(block: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(block).unwrap()
}

pub fn block_deserialize<'a, T>(bytes: &'a [u8]) -> &'a T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) .-> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

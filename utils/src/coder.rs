use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

pub fn block_serialize<T: ?Sized>(block: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(block).unwrap()
}

pub fn block_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: serde::de::Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

// test
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Test {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_should_works() {
        let test = Test {
            name: "test".to_string(),
        };
        let result = block_serialize(&test);
        let result_deserialize: Test = block_deserialize(&result[..]);
        assert_eq!(test, result_deserialize);
    }

    #[test]
    fn hash_works() {
        let test = Test {
            name: "test".to_string(),
        };
        let result = block_serialize(&test);
        let hash: String = get_hash(&result[..]);
        assert_eq!(
            hash,
            "c3e53ee0e3b2655fb8658831847e5685a30e2c5a5ea83f675b97abe7cb1fc599".to_string()
        );
    }
}

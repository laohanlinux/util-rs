use hex;
use sha3::{Sha3_256, Digest};
use ethereum_types::H256;
use keccak_hash::keccak;

pub fn to_hex<T: AsRef<[u8]>>(data: T) -> String{
    hex::encode(data)
}

pub fn to_sha3(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::default();
    hasher.input(data);
    hasher.result().to_vec()
}

pub fn to_keccak<T: AsRef<[u8]>>(data: T) -> H256 {
    keccak(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn keccak_sha() {
        let v = vec![1, 2, 3];
        let sha3 = to_sha3(&v);
        let keccak = to_keccak(&v);
        writeln!(io::stdout(), "{:?}", sha3).unwrap();
        writeln!(io::stdout(), "{:?}", &keccak[0..32]).unwrap();
    }
}
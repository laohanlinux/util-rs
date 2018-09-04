use hex;
use sha3::{Sha3_256, Digest};

pub fn to_hex<T: AsRef<[u8]>>(data: T) -> String{
    hex::encode_upper(data)
}


pub fn to_sha3<T: AsRef<u8>>(data: T) -> Vec<u8> {
    let mut hasher = Sha3_256::default();
    hasher.input(data);
    hasher.result()
//    format!("0x{:x}", out)
}
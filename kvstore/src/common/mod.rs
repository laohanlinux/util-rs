use hex;
use sha3;

pub fn to_hex<T: AsRef<[u8]>>(data: T) -> String{
    hex::encode_upper(data)
}


pub fn to_sha3<T: AsRef<u8>>(data: T) -> Vec<u8> {
    let mut hasher = sha3::default();
    hasher.input(data);
    hasher.result()
//    format!("0x{:x}", out)
}
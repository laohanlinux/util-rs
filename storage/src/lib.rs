pub mod common;
pub mod crypto;
#[macro_use]
pub mod encoding;
pub mod ethkey;
pub mod mem;
#[macro_use]
pub mod storage;

extern crate ethereum_types;
extern crate keccak_hash;
extern crate secp256k1;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate byteorder;
extern crate chrono;
extern crate exonum_rocksdb as rocksdb;
extern crate hex;
extern crate rmp;
extern crate rmp_serde as rmps;
extern crate sha3;
extern crate uuid;
#[macro_use]
extern crate lazy_static;
extern crate parity_crypto;
extern crate rand;
extern crate rust_decimal;
extern crate rustc_hex;
extern crate tiny_keccak;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

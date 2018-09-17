pub mod storage;
pub mod crypto;
#[macro_use]
pub mod encoding;
pub mod common;
pub mod ethkey;
pub mod mem;

extern crate secp256k1;
extern crate ethereum_types;
extern crate keccak_hash;
extern crate serde;
#[macro_use]extern crate serde_derive;
#[macro_use]extern crate serde_json;
#[macro_use]extern crate failure;
extern crate exonum_rocksdb as rocksdb;
extern crate hex;
extern crate sha3;
extern crate byteorder;
extern crate chrono;
extern crate uuid;
extern crate rmp;
extern crate rmp_serde as rmps;
#[macro_use]extern crate lazy_static;
extern crate rand;
extern crate tiny_keccak;
extern crate rustc_hex;
extern crate parity_crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod storage;
pub mod crypto;
pub mod common;
pub mod encoding;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate exonum_rocksdb;
extern crate hex;
extern crate sha3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

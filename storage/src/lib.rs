pub mod storage;
pub mod crypto;
pub mod encoding;
pub mod common;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate exonum_rocksdb as rocksdb;
extern crate hex;
extern crate sha3;
extern crate byteorder;
extern crate chrono;
extern crate uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

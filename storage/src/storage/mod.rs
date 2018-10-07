pub mod db;
pub mod error;
pub mod hash;
//pub mod rocksdb;
//pub mod base_index;
//pub mod indexes_metadata;
#[macro_use]
pub mod keys;
#[macro_use]
pub mod values;

pub use self::error::Error;
pub use encoding;

/// A specialized `Result` type for I/O operations with storage.
pub type Result<T> = ::std::result::Result<T, Error>;

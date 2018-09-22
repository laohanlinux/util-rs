pub mod db;
pub mod error;
pub mod hash;
//pub mod rocksdb;
pub mod values;
pub mod keys;

pub use self::error::Error;

/// A specialized `Result` type for I/O operations with storage.
pub type Result<T> = ::std::result::Result<T, Error>;

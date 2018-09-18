use std::ops::{Deref, DerefMut};
use std::cmp::PartialEq;
use std::fmt;
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use secp256k1::{Message as SecpMessage, RecoverableSignature, RecoveryId, Error as SecpError};
use secp256k1::key::{SecretKey, PublicKey};
use ethereum_types::{H520, H256};
use super::{Secret, Public, SECP256K1, Error, Address};

pub const SIGNATURE_SIZE: usize = 65;
pub const SIGNATURE_R_SIZE: usize = 32;
pub const SIGNATURE_S_SIZE: usize = 32;

/// Signature encoded as RSV components
pub struct Signature([u8; SIGNATURE_SIZE]);

impl Signature {
    /// Get a slice into the `r` portion of the data.
    pub fn r(&self) -> &[u8] {
        &self.0[0..SIGNATURE_R_SIZE]
    }

    /// Get a slice into the `s` portion of the data.
    pub fn s(&self) -> &[u8] {
        &self.0[SIGNATURE_R_SIZE..(SIGNATURE_SIZE + SIGNATURE_S_SIZE)]
    }

    /// Get the recovery byte
    pub fn v(&self) -> u8 {
        self.0[64]
    }

    /// Encode the signature into RSV array (V altered to be in `Electrum` notation).
    pub fn into_electrum(mut self) -> [u8; SIGNATURE_SIZE] {
        self.0[64] += 27;
        self.0
    }

    /// Parse bytes as a signature encoded as RSV (V in "Electrum" notation).
    /// May be return empty (invalid) signature if given data has invalid length.
    pub fn from_electrum(data: &[u8]) -> Self {
        if data.len() != SIGNATURE_SIZE || data[64] < 27 {
            // fallback to empty (invalid) signature
            return Signature::default();
        }

        let mut sig = [0u8; SIGNATURE_SIZE];
        sig.copy_from_slice(data);
        sig[64] -= 27;
        Signature(sig)
    }
}
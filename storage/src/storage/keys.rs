// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unsafe_code)]

//! A definition of `StorageKey` trait and implementations for common types.

use crypto::{Hash, CryptoHash, hash};
use ::ethkey::{Public, Signature, Random, Generator};

use rmps::{Serializer, Deserializer};
use rmps::decode::Error;

use std::mem;
use std::borrow::Cow;
use std::io::Cursor;

pub trait StorageKey: ToOwned {
    /// Returns the size of the serialized key in bytes.
    fn size(&self) -> usize;

    /// Serializes the key into the specified buffer of bytes.
    ///
    /// The caller must guarantee that the size of the buffer is equal to the precalculated size
    /// of the serialized key.
    // TODO: Should be unsafe? (ECR-174)
    fn write(&self, buffer: &mut [u8]);

    /// Deserializes the key from the specified buffer of bytes.
    // TODO: Should be unsafe? (ECR-174)
    fn read(buffer: &[u8]) -> Self::Owned;
}

/// No-op implementation.
impl StorageKey for () {
    fn size(&self) -> usize {
        0
    }

    fn write(&self, _buffer: &mut [u8]) {
        // no-op
    }

    fn read(_buffer: &[u8]) -> Self::Owned {
        ()
    }
}

impl StorageKey for u8 {
    fn size(&self) -> usize {
        1
    }

    fn write(&self, buffer: &mut [u8]) {
        buffer[0] = *self
    }

    fn read(buffer: &[u8]) -> Self::Owned {
        buffer[0]
    }
}

/// Uses encoding with the values mapped to `u8`
/// by adding the corresponding constant (`128`) to the value.
impl StorageKey for i8 {
    fn size(&self) -> usize {
        1
    }

    fn write(&self, buffer: &mut [u8]) {
        buffer[0] = self.wrapping_add(i8::min_value()) as u8;
    }

    fn read(buffer: &[u8]) -> Self::Owned {
        buffer[0].wrapping_sub(i8::min_value() as u8) as i8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn mannul() {
        let keypair = Random.generate().unwrap();
    }
}
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

//! An implementation of base index with most common features.
//!
//! The `BaseIndex` structure is not intended for direct use, rather it is the
//! basis for building other types of indices. The given section contains methods
//! related to `BaseIndex` and the iterator over the items of this index.

// spell-checker:ignore subprefix

use std::{borrow::Cow, marker::PhantomData};

use super::db::{Fork, Iter, Snapshot};
use super::keys::StorageKey;
use super::values::StorageValue;
//use storage::indexes_metadata::{self, IndexType, INDEXES_METADATA_TABLE_NAME};

/// Basic struct for all indices that implements common features.
///
/// This structure is not intended for direct use, rather it is the basis for building other types
/// of indices.
///
/// `BaseIndex` requires that keys should implement the [`StorageKey`] trait and
/// values should implement the [`StorageValue`] trait. However, this structure
/// is not bound to specific types and allows the use of *any* types as keys or values.
///
/// [`StorageKey`]: ../trait.StorageKey.html
/// [`StorageValue`]: ../trait.StorageValue.html
#[derive(Debug)]
pub struct BaseIndex<T> {
    name: String,
    is_family: bool,
    index_id: Option<Vec<u8>>,
    is_mutable: bool,
    //index_type: IndexType,
    view: T,
}

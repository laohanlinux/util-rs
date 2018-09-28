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
use storage::indexes_metadata::{self, IndexType, INDEXES_METADATA_TABLE_NAME};

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
    index_type: IndexType,
    view: T,
}

/// An iterator over the entries of a `BaseIndex`.
///
/// This struct is created by the [`iter`] or
/// [`iter_from`] method on [`BaseIndex`]. See its documentation for details.
///
/// [`iter`]: struct.BaseIndex.html#method.iter
/// [`iter_from`]: struct.BaseIndex.html#method.iter_from
/// [`BaseIndex`]: struct.BaseIndex.html
pub struct BaseIndexIter<'a, K, V> {
    base_iter: Iter<'a>,
    base_prefix_len: usize,
    index_id: Vec<u8>,
    ended: bool,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<T> BaseIndex<T>
where
    T: AsRef<dyn Snapshot>,
{
    /// Creates a new index representation based on the name and storage view.
    ///
    /// Storage view can be specified as [`&Snapshot`] or [`&mut Fork`]. In the first case, only
    /// immutable methods are available. In the second case, both immutable and mutable methods are
    /// available.
    ///
    /// [`&Snapshot`]: ../trait.Snapshot.html
    /// [`&mut Fork`]: ../struct.Fork.html
    pub fn new<S: AsRef<str>>(index_name: S, index_type: IndexType, view: T) -> Self {
        assert_valid_name(&index_name);

        let is_family = false;
        indexes_metadata::assert_index_type(
            index_name.as_ref(),
            index_type,
            is_family,
            view.as_ref(),
        );

        Self {
            name: index_name.as_ref().to_string(),
            is_family,
            index_id: None,
            is_mutable: false,
            index_type,
            view,
        }
    }
}

/// A function that validators and index name. Allowable characters in name:  ASCII characters,
/// digists and underscores.
fn is_valid_name<S: AsRef<str>>(name: S) -> bool {
    name.as_ref().as_bytes().iter().all(|c: &u8| match *c {
        48...57 | 65...90 | 97...122 | 95 | 46 => true,
        _ => false,
    })
}

/// Calls the `is_valid_name` function with the given name and panics if it returns `false`.
fn assert_valid_name<S: AsRef<str>>(name: S) {
    if !is_valid_name(name) {
        panic!("Wrong characters using in name. Use: a-zA-Z0-9 and _");
    }
}

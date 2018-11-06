//! Tests for `std::collections::HashMap`
use arbitrary::*;
use std::hash::{BuildHasher, Hash, Hasher};
use std::mem;

/// Build a [`TrulyAwfulHasher`]
///
/// This struct serves only to anchor a [`BuildHasher`]. It has no internal
/// mechanism.
pub struct BuildTrulyAwfulHasher {
    seed: u8,
}

impl BuildTrulyAwfulHasher {
    /// Construct a new `BuildTrulyAwfulHasher`
    ///
    /// The passed `seed` will be used as the initial seed of the
    /// [`TrulyAwfulHasher`]. See that type's documentation for details.
    pub fn new(seed: u8) -> Self {
        Self { seed }
    }
}

impl BuildHasher for BuildTrulyAwfulHasher {
    type Hasher = TrulyAwfulHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TrulyAwfulHasher::new(self.seed)
    }
}

/// A [`Hasher`] but one which is very bad at its job
///
/// The internal mechanism of `TrulyAwfulHasher` is very simple. The type
/// maintains a `hash_value: u8` which is updated on every call to
/// [`Hasher::write`]. How is it updated? The first byte is removed from the
/// input slice and wrappingly summed to `hash_value`. That is, even though the
/// `Hasher::finish` for this type will return a `u64` we know that the values
/// will be `[0, 256)`, all but guaranteeing hash-collisions for any user of
/// this hasher.
pub struct TrulyAwfulHasher {
    hash_value: u8,
}

impl TrulyAwfulHasher {
    /// Construct a new `TrulyAwfulHasher`
    ///
    /// The passed `seed` will be used as the initial value of the type's
    /// `hash_value`. See this type's documentation for details.
    fn new(seed: u8) -> Self {
        Self { hash_value: seed }
    }
}

impl Hasher for TrulyAwfulHasher {
    fn write(&mut self, bytes: &[u8]) {
        if let Some(byte) = bytes.first() {
            self.hash_value = self.hash_value.wrapping_add(*byte) % 8;
        }
    }

    fn finish(&self) -> u64 {
        u64::from(self.hash_value)
    }
}

/// A `HashMap<K, V>` model
///
/// This type mimics the semantics of a `HashMap<K, V>` while being 'obviously
/// correct' enough to serve as a `QuickCheck` model. The interface for the two
/// types is roughly equivalent, except in construction. This similarity allows
/// for `PropHashMap<K, V>` and `HashMap<K, V>` to be compared against one
/// another in a `QuickCheck` suite.
///
/// In actuality, `PropHashMap<K, V>` is a vector of `(K, V)`. The pairs are not
/// held in order so the operations against the map are extremely
/// inefficient. But, they are simple to implement and verify.
pub struct PropHashMap<K, V>
where
    K: Eq + Hash,
{
    data: Vec<(K, V)>,
}

impl<K, V> Default for PropHashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> PropHashMap<K, V>
where
    K: Eq + Hash,
{
    /// Construct a new `PropHashMap<K, V>`
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Get a value from the `PropHashMap<K, V>`, if one exists
    ///
    /// This is like to [`std::collections::HashMap::get`]
    pub fn get(&mut self, k: &K) -> Option<&V> {
        self.data.iter().find(|probe| probe.0 == *k).map(|e| &e.1)
    }

    /// Determine if the `PropHashMap` is empty
    ///
    /// This is like to [`std::collections::HashMap::is_empty`]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the length of the `PropHashMap`
    ///
    /// This is like to [`std::collections::HashMap::len`]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clear all contents of `PropHashMap`
    ///
    /// This is like to [`std::collections::HashMap::clear`]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Insert a value into `PropHashMap<K, V>`, returning the previous value if
    /// one existed
    ///
    /// This is like to [`std::collections::HashMap::insert`]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(e) = self.data.iter_mut().find(|probe| probe.0 == k) {
            return Some(mem::replace(&mut e.1, v));
        }
        self.data.push((k, v));
        None
    }

    /// Remove a value from `PropHashMap<K, V>` at the given key, returning the
    /// previous value if one existed
    ///
    /// This is like to [`std::collections::HashMap::remove`]
    pub fn remove(&mut self, k: &K) -> Option<V> {
        if let Some(idx) = self.data.iter().position(|probe| probe.0 == *k) {
            Some(self.data.swap_remove(idx).1)
        } else {
            None
        }
    }
}

/// The `Op<K, V>` defines the set of operations that are available against
/// `HashMap<K, V>` and `PropHashMap<K, V>`. Some map directly to functions
/// available on the types, others require a more elaborate interpretation
/// step.
#[derive(Clone, Debug)]
pub enum Op<K, V> {
    /// This operation triggers `std::collections::HashMap::shrink_to_fit`
    ShrinkToFit,
    /// This operation triggers `std::collections::HashMap::clear`
    Clear,
    /// This operation triggers `std::collections::HashMap::reserve`
    Reserve {
        /// Reserve `n` capacity elements
        n: usize,
    },
    /// This operation triggers `std::collections::HashMap::insert`
    Insert {
        /// The key to be inserted
        k: K,
        /// The value to be inserted
        v: V,
    },
    /// This operation triggers `std::collections::HashMap::remove`
    Remove {
        /// The key to be removed
        k: K,
    },
    /// This operation triggers `std::collections::HashMap::get`
    Get {
        /// The key to be removed
        k: K,
    },
}

impl<K, V> Arbitrary for Op<K, V>
where
    K: Clone + Send + Arbitrary,
    V: Clone + Send + Arbitrary,
{
    fn arbitrary<U>(u: &mut U) -> Result<Self, U::Error>
    where
        U: Unstructured + ?Sized,
    {
        // ================ WARNING ================
        //
        // `total_enum_fields` is a goofy annoyance but it should match
        // _exactly_ the number of fields available in `Op<K, V>`. If it
        // does not then we'll fail to generate `Op` variants for use in our
        // QC tests.
        let total_enum_fields = 6;
        let variant: u8 = Arbitrary::arbitrary(u)?;
        let op = match variant % total_enum_fields {
            0 => {
                let k: K = Arbitrary::arbitrary(u)?;
                let v: V = Arbitrary::arbitrary(u)?;
                Op::Insert { k, v }
            }
            1 => {
                let k: K = Arbitrary::arbitrary(u)?;
                Op::Remove { k }
            }
            2 => {
                let k: K = Arbitrary::arbitrary(u)?;
                Op::Get { k }
            }
            3 => Op::ShrinkToFit,
            4 => Op::Clear,
            5 => {
                let n: usize = Arbitrary::arbitrary(u)?;
                Op::Reserve { n }
            }
            _ => unreachable!(),
        };
        Ok(op)
    }
}

//! Tests for `std::collections::HashMap`
use std::hash::{BuildHasher, Hash, Hasher};
use std::mem::swap;

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
    fn write(&mut self, bytes: &[u8]) -> () {
        if let Some(byte) = bytes.first() {
            self.hash_value = self.hash_value.wrapping_add(*byte);
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
        if let Some(idx) = self.data.iter().position(|probe| probe.0 == *k) {
            Some(&(self.data[idx].1))
        } else {
            None
        }
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
    pub fn clear(&mut self) -> () {
        self.data.clear()
    }

    /// Insert a value into `PropHashMap<K, V>`, returning the previous value if
    /// one existed
    ///
    /// This is like to [`std::collections::HashMap::insert`]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(idx) = self.data.iter().position(|probe| probe.0 == k) {
            // TODO(blt) This violates the semantics of HashMap a
            // little. Specifically, we change out the key. HashMap documents
            // that it _does not_ update the key on every insert, important for
            // types that are Eq without actually being identical.
            //
            // TODO(blt) This is an important enough constraint that there
            // should be a property around this.
            let mut pair = (k, v);
            swap(&mut pair, &mut self.data[idx]);
            Some(pair.1)
        } else {
            self.data.push((k, v));
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
    use std::collections::HashMap;

    // The `Op<K, V>` defines the set of operations that are available against
    // `HashMap<K, V>` and `PropHashMap<K, V>`. Some map directly to functions
    // available on the types, others require a more elaborate interpretation
    // step. See `oprun` below for details.
    #[derive(Clone, Debug)]
    enum Op<K, V> {
        ShrinkToFit,
        CheckIsEmpty,
        CheckLen,
        CheckCapacity,
        Clear,
        Insert { k: K, v: V },
        Get { k: K },
    }

    impl<K: 'static, V: 'static> Arbitrary for Op<K, V>
    where
        K: Clone + Send + Arbitrary,
        V: Clone + Send + Arbitrary,
    {
        fn arbitrary<G>(g: &mut G) -> Op<K, V>
        where
            G: Gen,
        {
            let k: K = Arbitrary::arbitrary(g);
            let v: V = Arbitrary::arbitrary(g);
            // ================ WARNING ================
            //
            // `total_enum_fields` is a goofy annoyance but it should match
            // _exactly_ the number of fields available in `Op<K, V>`. If it
            // does not then we'll fail to generate `Op` variants for use in our
            // QC tests.
            let total_enum_fields = 7;
            let variant = g.gen_range(0, total_enum_fields);
            match variant {
                0 => Op::CheckIsEmpty,
                1 => Op::CheckLen,
                2 => Op::Insert { k, v },
                3 => Op::Get { k },
                4 => Op::ShrinkToFit,
                5 => Op::CheckCapacity,
                6 => Op::Clear,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn oprun() {
        fn inner(hash_seed: u8, capacity: usize, ops: Vec<Op<u16, u16>>) -> TestResult {
            let mut model: PropHashMap<u16, u16> = PropHashMap::new();
            let mut sut: HashMap<u16, u16, BuildTrulyAwfulHasher> =
                HashMap::with_capacity_and_hasher(capacity, BuildTrulyAwfulHasher::new(hash_seed));
            for op in &ops {
                match op {
                    Op::Clear => {
                        // Clearning a HashMap removes all elements but keeps
                        // the memory around for reuse. That is, the length
                        // should drop to zero but the capacity will remain the
                        // same.
                        let prev_cap = sut.capacity();
                        sut.clear();
                        model.clear();
                        assert_eq!(0, sut.len());
                        assert_eq!(sut.len(), model.len());
                        assert_eq!(prev_cap, sut.capacity());
                    }
                    Op::ShrinkToFit => {
                        // After a shrink the capacity may or may not shift from
                        // the passed arg `capacity`. But, the capacity of the
                        // HashMap should never grow after a shrink.
                        //
                        // Similarly, the length of the HashMap prior to a
                        // shrink should match the length after a shrink.
                        let prev_len = sut.len();
                        let prev_cap = sut.capacity();
                        // Note there is no model behaviour here
                        sut.shrink_to_fit();
                        assert_eq!(prev_len, sut.len());
                        assert!(sut.capacity() <= prev_cap);
                    }
                    Op::CheckCapacity => {
                        // `HashMap<K, V>` defines the return of `capacity` as
                        // being "the number of elements the map can hold
                        // without reallocating", noting that the number is a
                        // "lower bound". This implies that:
                        //
                        //  * the capacity must always be at least the arg
                        //    `capacity`
                        //  * the HashMap capacity must always be at least the
                        //    length of the model
                        assert!(sut.capacity() >= model.len());
                    }
                    Op::CheckIsEmpty => {
                        let model_res = model.is_empty();
                        let sut_res = sut.is_empty();
                        assert_eq!(model_res, sut_res);
                    }
                    Op::CheckLen => {
                        let model_res = model.len();
                        let sut_res = sut.len();
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Get { k } => {
                        let model_res = model.get(k);
                        let sut_res = sut.get(&k);
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Insert { k, v } => {
                        let model_res = model.insert(*k, *v);
                        let sut_res = sut.insert(*k, *v);
                        assert_eq!(model_res, sut_res);
                    }
                }
            }
            TestResult::passed()
        }
        QuickCheck::new().quickcheck(inner as fn(u8, usize, Vec<Op<u16, u16>>) -> TestResult)
    }
}

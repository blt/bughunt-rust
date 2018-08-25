#[macro_use]
extern crate afl;
extern crate arbitrary;
extern crate bughunt_rust;

use arbitrary::*;
use bughunt_rust::stdlib::collections::hash_map::*;
use std::collections::HashMap;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut ring = RingBuffer::new(data, 4048).unwrap();

            let hash_seed: u8 = Arbitrary::arbitrary(&mut ring).unwrap();
            let capacity: usize = Arbitrary::arbitrary(&mut ring).unwrap();
            let total_ops: u16 = Arbitrary::arbitrary(&mut ring).unwrap();

            let mut model: PropHashMap<u16, u16> = PropHashMap::new();
            let mut sut: HashMap<u16, u16, BuildTrulyAwfulHasher> =
                HashMap::with_capacity_and_hasher(capacity, BuildTrulyAwfulHasher::new(hash_seed));

            for _ in 0..total_ops {
                let op: Op<u16, u16> = Arbitrary::arbitrary(&mut ring).unwrap();
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
                        // NOTE There is no model behaviour here
                        //
                        // After a shrink the capacity may or may not shift from
                        // the passed arg `capacity`. But, the capacity of the
                        // HashMap should never grow after a shrink.
                        //
                        // Similarly, the length of the HashMap prior to a
                        // shrink should match the length after a shrink.
                        let prev_len = sut.len();
                        let prev_cap = sut.capacity();
                        sut.shrink_to_fit();
                        assert_eq!(prev_len, sut.len());
                        assert!(sut.capacity() <= prev_cap);
                    }
                    Op::Get { k } => {
                        let model_res = model.get(&k);
                        let sut_res = sut.get(&k);
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Insert { k, v } => {
                        let model_res = model.insert(k, v);
                        let sut_res = sut.insert(k, v);
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Remove { k } => {
                        let model_res = model.remove(&k);
                        let sut_res = sut.remove(&k);
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Reserve { n } => {
                        // NOTE There is no model behaviour here
                        //
                        // When we reserve we carefully check that we're not
                        // reserving into overflow territory. When
                        // `#![feature(try_reserve)]` is available we can
                        // make use of `try_reserve` on the SUT
                        if sut.capacity().checked_add(n).is_some() {
                            sut.reserve(n);
                        } // else { assert!(sut.try_reserve(*n).is_err()); }
                    }
                }
                // Check invariants
                //
                // `HashMap<K, V>` defines the return of `capacity` as
                // being "the number of elements the map can hold
                // without reallocating", noting that the number is a
                // "lower bound". This implies that:
                //
                //  * the HashMap capacity must always be at least the
                //    length of the model
                assert!(sut.capacity() >= model.len());
                // If the SUT is empty then the model must be.
                assert_eq!(model.is_empty(), sut.is_empty());
                // The length of the SUT must always be exactly the length of
                // the model.
                assert_eq!(model.len(), sut.len());
            }
        })
    }
}

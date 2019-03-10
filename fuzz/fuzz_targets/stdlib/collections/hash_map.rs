#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arbitrary;
extern crate bughunt_rust;

use arbitrary::*;
use bughunt_rust::stdlib::collections::hash_map::*;
use std::collections::HashMap;

fuzz_target!(|data: &[u8]| {
    if let Ok(mut ring) = FiniteBuffer::new(data, 16_384) {
        let hash_seed: u8 = if let Ok(hs) = Arbitrary::arbitrary(&mut ring) {
            hs
        } else {
            return;
        };
        // Why is capacity not usize? We're very likely to request a
        // capacity so large that the HashMap cannot allocate enough
        // slots to store them, resulting in a panic when we call
        // `with_capacity_and_hasher`. This is a crash, but an
        // uninteresting one.
        //
        // We also request a low-ish capacity, all but guaranteeing we'll
        // force reallocation during execution.
        //
        // See note on [`hash_map::Op::Reserve`] for details
        let capacity: u8 = if let Ok(cap) = Arbitrary::arbitrary(&mut ring) {
            cap
        } else {
            return;
        };

        let mut model: PropHashMap<u16, u16> = PropHashMap::new();
        let mut sut: HashMap<u16, u16, BuildTrulyAwfulHasher> = HashMap::with_capacity_and_hasher(
            capacity as usize,
            BuildTrulyAwfulHasher::new(hash_seed),
        );

        while let Ok(op) = Arbitrary::arbitrary(&mut ring) {
            match op {
                Op::Clear => {
                    // Clearing a HashMap removes all elements but keeps
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
                    if sut.capacity().checked_add(n as usize).is_some() {
                        sut.reserve(n as usize);
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
    }
});

#[macro_use]
extern crate honggfuzz;
extern crate arbitrary;
extern crate bughunt_rust;

use arbitrary::*;
use bughunt_rust::stdlib::collections::vec_deque::*;
use std::collections::VecDeque;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(mut ring) = FiniteBuffer::new(data, 65_563) {
                let capacity: u8 = if let Ok(cap) = Arbitrary::arbitrary(&mut ring) {
                    cap
                } else {
                    return;
                };
                let mut model: PropVecDeque<u8> = PropVecDeque::new();
                let mut sut: VecDeque<u8> = VecDeque::with_capacity(capacity as usize);
                while let Ok(op) = Arbitrary::arbitrary(&mut ring) {
                    match op {
                        Op::PushBack(t) => {
                            sut.push_back(t);
                            model.push_back(t);
                        }
                        Op::PopBack => {
                            let sut_res = sut.pop_back();
                            let model_res = model.pop_back();
                            assert_eq!(sut_res, model_res);
                        }
                    }
                    // Check invariants
                    //
                    // `VecDeque<T>` defines the return of `capacity` as being
                    // "the number of elements the map can hold without
                    // reallocating". Unlike `HashMap<K, V>` there is no
                    // discussion of bounds. This implies that:
                    //
                    // * the VecDeque capacity must always be at least the
                    // length of the model
                    assert!(sut.capacity() >= model.len());
                    // The length of the SUT must always be exactly the length
                    // of the model.
                    assert_eq!(sut.len(), model.len());
                    // If the SUT is empty then the model must also be.
                    assert_eq!(sut.is_empty(), model.is_empty());
                }
            }
        })
    }
}

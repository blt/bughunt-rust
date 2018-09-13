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
                        Op::Clear => {
                            // Clearning a VecDeque removes all elements but keeps
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
                            // VecDeque should never grow after a shrink.
                            //
                            // Similarly, the length of the VecDeque prior to a
                            // shrink should match the length after a shrink.
                            let prev_len = sut.len();
                            let prev_cap = sut.capacity();
                            sut.shrink_to_fit();
                            assert_eq!(prev_len, sut.len());
                            assert!(sut.capacity() <= prev_cap);
                        }
                        Op::PushBack(t) => {
                            sut.push_back(t);
                            model.push_back(t);
                        }
                        Op::PushFront(t) => {
                            sut.push_front(t);
                            model.push_front(t);
                        }
                        Op::PopFront => {
                            let sut_res = sut.pop_front();
                            let model_res = model.pop_front();
                            assert_eq!(sut_res, model_res);
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
                    // The front of the SUT must always be equivalent to the
                    // front of the model.
                    assert_eq!(sut.front(), model.front());
                    // The back of the SUT must always be equivalent to the
                    // back of the model.
                    assert_eq!(sut.back(), model.back());
                }
            }
        })
    }
}

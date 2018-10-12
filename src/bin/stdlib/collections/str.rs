#[macro_use]
extern crate afl;
extern crate arbitrary;
extern crate bughunt_rust;

use arbitrary::*;
use bughunt_rust::stdlib::collections::hash_map::*;
use std::collections::HashMap;

fn main() {
    fuzz!(|data: &[u8]| if let Ok(mut ring) = FiniteBuffer::new(data, 65_563) {})
}

#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate arbitrary;
extern crate bughunt_rust;

use arbitrary::*;
use std::{panic, str};

fuzz_target!(|data: &[u8]| {
    if let Ok(mut buf) = FiniteBuffer::new(data, 4_096) {
        let vs: Vec<u8> = if let Ok(vs) = Arbitrary::arbitrary(&mut buf) {
            vs
        } else {
            return;
        };

        let s: &str = if let Ok(s) = str::from_utf8(&vs) {
            s
        } else {
            return;
        };

        // According to the docs for `str::repeat`:
        //
        // > This function will panic if the capacity would overflow.
        //
        // I take that to mean if the length of the str multiplied by
        // 'repeats' is greater than usize. The trick is though, we
        // don't know how much memory is actually available on-system:
        // any largish allocation might panic before reaching usize
        // territory. As a result, we request an arbitrary u16 and bump
        // that up to word size.
        //
        // I could, I guess, wrap the call to `str::repeat` in a
        // `catch_unwind` but that'll potentially mask other panics, not
        // just memory related ones.
        let repeats: u16 = if let Ok(rpts) = Arbitrary::arbitrary(&mut buf) {
            rpts
        } else {
            return;
        };
        let repeats: usize = repeats as usize;

        if let Some(rpt_len) = s.len().checked_mul(repeats) {
            let res = s.repeat(repeats);
            assert_eq!(res.len(), rpt_len);
        }
    }
});

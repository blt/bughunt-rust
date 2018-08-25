extern crate arbitrary;
#[deny(warnings)]
#[deny(bad_style)]
#[deny(missing_docs)]
#[deny(future_incompatible)]
#[deny(nonstandard_style)]
#[deny(rust_2018_compatibility)]
#[deny(rust_2018_idioms)]
#[deny(unused)]
#[cfg_attr(feature = "cargo-clippy", deny(clippy))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_perf))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_style))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_complexity))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_correctness))]
#[cfg_attr(feature = "cargo-clippy", deny(clippy_cargo))]
// We allow 'stuttering' as the structure of the project will mimic that of
// stdlib. For instance, QC tests for HashMap will appear in a module called
// `hash_map`.
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub mod stdlib;

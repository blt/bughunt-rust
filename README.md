# BugHunt, Rust

[![Build Status](https://travis-ci.com/blt/bughunt-rust.svg?branch=master)](https://travis-ci.com/blt/bughunt-rust)

This project is aiming to provide "stateful" QuickCheck models for Rust's
standard library. That is, we build up a random list of operations against an
abstract data type, an "obviously correct" model of that ADT and apply the
operations to both the model and the reference implementation of the data
type. If the model and reference implementation differ in any way then that's a
good sign there's a bug to be diagnosed and reported. This is _different_ from
fuzzing in that we're interested in higher-level behaviour of data
structures--their "properties"--and aren't necessarily looking for
crashes. (That said, "do not crash the program" is a pretty good property for
most data structures.)

We're inspired by the work [**@jlouis**](https://github.com/jlouis) did in the
Erlang community to detect subtle bugs in that language's map implementation and
[**@shnatsel**](https://github.com/Shnatsel)'s recent work fuzzing Rust crates
for crashes.

## Running the Suite

Running the tests takes a little leg work. The project performs model-based
fuzzing, which means the tests are driven by a fuzzer, cargo-fuzz (libFuzzer) in
particular. We've written about the general approach
[here](https://blog.troutwine.us/2018/10/08/hunting-for-bugs-in-rust/). Since
this post we've switch from AFL to libFuzzer but the broad details remain the
same.

The available targets are listed out in [`fuzz/Cargo.toml`], the binaries of the
project. Say you want to run the `str::repeat` target. Make sure you've got
cargo-fuzz installed by running `cargo install cargo-fuzz`.

```
> cargo fuzz run str_repeat
```

A reasonable test run will take hours and as configured the above run will
execute forever. Give the flag `--help` to `cargo fuzz` to see its options
relating to runtime constriction, corpus definition etc.

### Why does this run outside of Rust itself?

Well! I'm not sure that bundling these long-running tests into the Rust compiler
project is something anyone would go for and, working here as an external
project, we can avoid needing to fiddle with toolchains and longish build
cycles. Downside is, the std data structures we're testing don't have any
sanitizers turned on etc on account of the project is run against the usual Rust
release.

## Contributing

Writing QuickCheck models can be slow work and contributions are _very_ welcome,
either introducing new models into the project or extending existing ones. We
have an experimental [clusterfuzz](https://github.com/google/clusterfuzz) setup
running and if you have credits to donate that would be most welcome. I intend
to document project balances, money needs once they are clear.

### Would you take CI help?

Yes! Right now we have a folder `ci/` which has the build scripts used in
`.travis.yml`. We're producing test binaries and feeding them directly into the
clusterfuzz setup the project has. Speaking of, I'll be adding configuration for
that cluster to this repository in the coming days.

Any improvements in the build pipeline, clusterfuzz configuration are most
welcome.

### Would you take documentation help?

Yes!

## Hey, how can I learn more?

Randomized testing is a touch esoteric but there's a lot of reading material
available (itself a problem, kind of). In no certain order:

* ["QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs"](https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf)
* ["Breaking Erlang Maps #1"](https://medium.com/@jlouis666/breaking-erlang-maps-1-31952b8729e6)
* ["How Rust’s standard library was vulnerable for years and nobody noticed"](https://medium.com/@shnatsel/how-rusts-standard-library-was-vulnerable-for-years-and-nobody-noticed-aebf0503c3d6)
* ["PropEr Testing"](https://propertesting.com/)
* ["Moonconf Papers"](https://blog.troutwine.us/2016/05/26/moonconf-papers/)
* ["Hybrid Fuzz Testing:Discovering Software Bugs viaFuzzing and Symbolic Execution"](http://reports-archive.adm.cs.cmu.edu/anon/2012/CMU-CS-12-116.pdf)
* ["QuickFuzz: An Automatic Random Fuzzer for Common File Formats"](https://people.seas.harvard.edu/~pbuiras/publications/QFHaskell2016.pdf)
* ["Angora: Efficient Fuzzing by Principled Search"](http://web.cs.ucdavis.edu/~hchen/paper/chen2018angora.pdf)

I, blt, am also happy to answer questions over email. I'm brian@troutwine.us.

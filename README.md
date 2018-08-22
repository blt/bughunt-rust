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

Getting the QuickCheck models running is straightforward. Clone this repository and run:

```
> cargo test
```

This should finish fairly rapidly and, sadly, won't be a comprehensive
search. The [QuickCheck we use](https://github.com/BurntSushi/quickcheck/) has
fairly low default generator settings, leading to a small-sized (but fast!)
state space search. An effective run will need to _drastically_ increase the
available state space. Here's a run that executes each model test 100 times:

```
> time cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running target/debug/deps/bughunt_rust-e1e71a34753653e7

running 1 test
test stdlib::collections::hash_map::test::oprun ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests bughunt-rust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test  0.17s user 0.27s system 88% cpu 0.495 total
```

Note this is us running `cargo test` again; the default is to run 100
tests. Here's 10,000 times:

```
time QUICKCHECK_TESTS=10000 cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running target/debug/deps/bughunt_rust-e1e71a34753653e7

running 1 test
test stdlib::collections::hash_map::test::oprun ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests bughunt-rust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

QUICKCHECK_TESTS=10000 cargo test  1.82s user 0.32s system 97% cpu 2.196 total
```

QuickCheck also has a notion of generator 'size', the details of which is beyond
the scope of this document. I promise, you're going to want to increase that
from the default too. Here's a generator size of 1000, total tests 10,000:

```
> time QUICKCHECK_GENERATOR_SIZE=1000 QUICKCHECK_TESTS=10000 cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running target/debug/deps/bughunt_rust-e1e71a34753653e7

running 1 test
test stdlib::collections::hash_map::test::oprun ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests bughunt-rust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

QUICKCHECK_GENERATOR_SIZE=1000 QUICKCHECK_TESTS=10000 cargo test  25.60s user 1.05s system 97% cpu 27.315 total
```

Slower each time but, potentially, more comprehensive. A reasonable test run
will take hours. (In fact, a suitable change to Rust QuickCheck would be to bind
its total tests not on number of tests but on duration of testing. Production
QuickCheck systems allow you to configure them to run all night and send a
report in the morning.)

### Why does this run outside of Rust itself?

Well! I'm not sure that bundling QuickCheck into the Rust compiler project is
something anyone would go for and, working here as an external project, we can
avoid needing to fiddle with toolchains and longish build cycles. Downside is,
the std data structures we're testing don't have any sanitizers turned on etc on
account of the project is run against the usual Rust releases.

## Contributing

Writing QuickCheck models can be slow work and contributions are _very_ welcome,
either introducing new models into the project or extending existing ones. Once
the project is a little more advanced donations of computing resources will
also be welcome. Writing QuickCheck models can be slow but, boy, running them is
no joke.

### Would you take fuzz tests?

Yes! There's a fair deal of overlap with writing effective fuzzers and writing
effective QuickCheck models. This project will happily take fuzz code. Or, if
someone could contrive to combine a fuzzer with a shrinking step this project
will jump on that in a heartbeat.

### Would you take CI help?

Yes! It'd be really nifty if this project could be run automatically against
every nightly, for instance, and flag when issues are discovered.

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

I, blt, am also happy to answer questions over email. I'm brian@troutwine.us.

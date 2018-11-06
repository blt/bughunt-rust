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
fuzzing, which means the tests are driven by a fuzzer, AFL in particular. We've
written about the general approach
[here](https://blog.troutwine.us/2018/10/08/hunting-for-bugs-in-rust/).

The available targets are listed out in [`Cargo.toml`], the binaries of the
project. Say you want to run the `str::repeat` target. Make sure you've got AFL
installed by running `cargo install afl`. That done, create input and output
directories for the fuzzer. The input directory influences what the fuzzer
initially uses to populate it's testcase pool. The output directory will hold
crashes, timeout etc data. Inputs have a huge influence on the running behaviour
of a target but it's not straightforward to know what input you should
supply. This is, uh, an open area of research.

We'll create an input directory filled with not-so-great data

```
> mkdir -p /tmp/repeat/in
> date >> /tmp/repeat/in/0000
> date >> /tmp/repeat/in/0001
> date >> /tmp/repeat/in/0002
> date >> /tmp/repeat/in/0003
```

and an output directory:

```
> mkdir -p /tmp/repeat/out
```

You can place these anywhere on disk you'd like. This is just an example. Okay,
from the root of the project:

```
> cargo build
> cargo afl fuzz -i /tmp/repeat/in -o /tmp/repeat/out/ target/debug/str_repeat
```

A reasonable test run will take hours. With the flags used above the run will
proceed indefinitely. Please note that AFL is single-threaded and to exploit
multi-core systems you'll need to spawn additional worker processes. This is
documented in the [AFL
project](https://github.com/mirrorer/afl/blob/master/docs/parallel_fuzzing.txt).

### Why does this run outside of Rust itself?

Well! I'm not sure that bundling these long-running tests into the Rust compiler
project is something anyone would go for and, working here as an external
project, we can avoid needing to fiddle with toolchains and longish build
cycles. Downside is, the std data structures we're testing don't have any
sanitizers turned on etc on account of the project is run against the usual Rust
releases.

## Contributing

Writing QuickCheck models can be slow work and contributions are _very_ welcome,
either introducing new models into the project or extending existing ones. Once
the project is a little more advanced donations of computing resources will
also be welcome. Writing QuickCheck models can be slow but, boy, running them is
no joke.

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

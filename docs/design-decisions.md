# Design Decisions

Several high-level decision have lead to the current design of `mutagen`. 

## Opt-in

Mutagen is implemented as a procedural macro, which means that only code annotated with `#[mutate]` is considered for mutations. This is a limitation but also a great feature (see warnings in the `readme`).

## Compile-once

This library is designed to be fast. We cannot afford re-compiling the test suite for every mutation. This means that all mutations have to be baked in at compile-time. This means we must avoid mutations that break the code in a way that it no longer compiles.

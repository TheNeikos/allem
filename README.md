# Generate different values of your data types

[![crates.io](https://img.shields.io/crates/v/allem.svg)](https://crates.io/crates/allem)
[![docs](https://docs.rs/allem/badge.svg)](https://docs.rs/allem/latest/allem/)


`allem` offers users the ability to easily generate lists of values that they consider
interesting. It was created with testing serialization and deserialization tests in mind, but
is not restricted to it.

The main trait of this crate is [Alles] which has a single function [Alles::generate] to get a
stream of values of whatever type implements it.

To implement it for your own types you can use the [Alles](derive@Alles) derive macro. Check it out for
documentation!


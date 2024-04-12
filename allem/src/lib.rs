//! # Generate different values of your data types
//!
//! This crate offers users the ability to easily generate lists of values that they consider
//! interesting. It was created with testing serialization and deserialization tests in mind, but
//! is not restricted to it.
//!
//! The main trait of this crate is [Alles] which has a single function [Alles::generate] to get a
//! stream of values of whatever type implements it.
//!
//! To implement it for your own types you can use the [Alles](derive@Alles) derive macro. Check it out for
//! documentation!
pub use allem_derive::Alles;

/// Trait to generate different variants of a type
///
/// The purpose of `Alles` is to, ironically, not generate _every_ permutation of a given type.
/// Instead, it is meant to cover most 'interesting' values. Where 'interesting' can be defined by
/// the user.
///
/// The easiest way to implement [Alles] is with the derive attribute.
pub trait Alles: Sized {
    /// A finite iterator of elements of this type
    fn generate() -> impl Iterator<Item = Self> + Clone;
}

impl Alles for u8 {
    /// The default impl for u8 returns a list of interesting values, mostly primes and maximums
    fn generate() -> impl Iterator<Item = Self> + Clone {
        [
            0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
            79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
            173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 255,
        ]
        .into_iter()
    }
}

impl Alles for i8 {
    /// The default impl for i8 returns a list of interesting values, mostly primes and maximums
    fn generate() -> impl Iterator<Item = Self> + Clone {
        [
            0, -1, -2, -3, -5, -7, -11, -13, -17, -19, -23, -29, -31, -37, -41, -43, -47, -53, -59,
            -61, -67, -71, -73, -79, -83, -89, -97, -101, -103, -107, -109, -113, -127, 1, 2, 3, 5,
            7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
            101, 103, 107, 109, 113, 127,
        ]
        .into_iter()
    }
}

impl<A: Alles + Clone> Alles for Option<A> {
    fn generate() -> impl Iterator<Item = Self> + Clone {
        std::iter::once(None).chain(A::generate().map(Some))
    }
}

#[doc(hidden)]
pub mod private {
    pub use itertools::iproduct;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct Foo {
        name: u8,
        age: i8,
    }

    impl Alles for Foo {
        fn generate() -> impl Iterator<Item = Self> + Clone {
            let name = u8::generate();
            let age = i8::generate();

            itertools::iproduct!(name, age).map(|(name, age)| Foo { name, age })
        }
    }

    #[test]
    fn check_generation() {
        let foo = Foo::generate();

        println!("{:?}", foo.size_hint());
    }
}

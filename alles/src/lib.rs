pub use alles_derive::Alles;

/// Trait that allows generating all useful variants of a type
pub trait Alles: Sized {
    fn generate() -> impl Iterator<Item = Self> + Clone;
}

impl Alles for u8 {
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

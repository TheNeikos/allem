#![allow(dead_code)]
use allem::Alles;

#[derive(Debug, Clone, Alles)]
struct Foo;

#[derive(Debug, Clone, Alles)]
struct Test {}

#[derive(Debug, Clone, Alles)]
struct Test2 {
    foo: u8,
    bar: i8,
}

#[derive(Debug, Clone, Alles)]
struct Test3();

#[derive(Debug, Clone, Alles)]
struct Test4(u8, i8);

#[derive(Debug, Clone, Alles)]
struct Test5<T: Alles + Clone> {
    foo: T,
    bar: u8,
}

#[derive(Debug, Clone, Alles)]
enum Test6 {
    Foo,
    Bar {
        #[alles(and_values = [-0, 2])]
        bar: i8,
    },
    Baz {
        #[alles(with_values = ["Test", "Foo"], and_values = ["bar"])]
        name: String,
    },
    Frob(#[alles(and_values = [23])] i8),
    Frab {
        #[alles(with_default)]
        val: bool,
    },
}

#[test]
fn check_impls() {
    let cnt = Test2::generate().count();
    let cnt2 = Test4::generate().count();

    assert_eq!(cnt, cnt2);

    let i8cnt = i8::generate().count();
    let cnt3 = Test6::generate().count();

    assert_eq!(i8cnt + 1 + 2 + 2 + 1 + i8cnt + 1 + 1, cnt3);
}

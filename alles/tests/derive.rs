#![allow(dead_code)]
use alles::Alles;

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
        bar: i8,
    },
    Baz {
        #[alles(with_values = ["Test", "Foo"])]
        name: String,
    },
}

#[test]
fn check_impls() {
    let cnt = Test2::generate().count();
    let cnt2 = Test4::generate().count();

    assert_eq!(cnt, cnt2);

    let i8cnt = i8::generate().count();
    let cnt3 = Test6::generate().count();

    assert_eq!(i8cnt + 1 + 2, cnt3);
}

#[actix_prost_macros::serde]
struct Foo {
    a: i32,
    b: u64,
}

#[test]
fn works() {
    serde_json::to_string(&Foo { a: 3, b: 4 }).unwrap();
}

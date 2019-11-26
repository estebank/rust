// compile-flags: --json=diagnostic-short --error-format=json
// FIXME: rustfix can't apply multipart suggestions, instead we look at the json instead.

enum Foo {
    enum Bar { Baz }, //~ ERROR enum cannot be nested inside `enum`
    struct Quux { field: u8 }, //~ ERROR struct cannot be nested inside `enum`
    union Wibble { field: u8 }, //~ ERROR union cannot be nested inside `enum`
    Bat,
    struct Trop { field: u8 } //~ ERROR struct cannot be nested inside `enum`
}

struct S {
    fn foo() {}, //~ ERROR function cannot be nested inside `struct`
    field: u8,
}

fn main() { }

enum Foo {
    enum Bar { Baz }, //~ ERROR enum cannot be nested inside `enum`
    struct Quux { field: u8 }, //~ ERROR struct cannot be nested inside `enum`
    union Wibble { field: u8 }, //~ ERROR union cannot be nested inside `enum`
    Bat,
}

fn main() { }

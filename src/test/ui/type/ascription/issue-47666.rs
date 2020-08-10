fn main() {
    let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
}
fn bar() {
    let _ = Foo:A; //~ ERROR expected value, found enum `Foo`
    //~^ ERROR cannot find type `A` in this scope
    let _ = Foo:B(2); //~ ERROR expected type
}
fn baz() {
    let _ = Foo:C { x: 1 }; //~ ERROR expected one of
}

enum Foo {
    A,
    B(i32),
    C {
        x: i32,
    },
}

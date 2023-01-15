// Tests that a suggestion is issued if the user wrote a colon instead of
// a path separator in a match arm.

mod qux {
    pub enum Foo {
        Bar,
        Baz,
    }
}

use qux::Foo;

fn f() -> Foo { Foo::Bar }

fn g1() {
    match f() {
        Foo:Bar => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: cannot find type `Bar` in this scope [E0412]
        //~| HELP: there is an enum variant `Foo::Bar`; try using the variant's enum
        _ => {}
    }
    match f() {
        qux::Foo:Bar => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: cannot find type `Bar` in this scope [E0412]
        //~| HELP: there is an enum variant `Foo::Bar`; try using the variant's enum
        //~| ERROR: expected unit struct, unit variant or constant, found enum `qux::Foo` [E0532]
        _ => {}
    }
    match f() {
        qux:Foo::Baz => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: expected type, found variant `Foo::Baz` [E0573]
        //~| HELP: try using the variant's enum
        _ => {}
    }
    match f() {
        qux: Foo::Baz if true => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: expected type, found variant `Foo::Baz` [E0573]
        //~| HELP: try using the variant's enum
        _ => {}
    }
    if let Foo:Bar = f() {
    //~^ ERROR: expected one of
    //~| HELP: maybe write a path separator here
    //~| ERROR: cannot find type `Bar` in this scope [E0412]
    //~| HELP: there is an enum variant `Foo::Bar`; try using the variant's enum
    }
}

fn g1_neg() {
    match f() {
        ref qux: Foo::Baz => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: expected type, found variant `Foo::Baz` [E0573]
        //~| HELP: try using the variant's enum
        _ => {}
    }
}

fn g2_neg() {
    match f() {
        mut qux: Foo::Baz => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: expected type, found variant `Foo::Baz` [E0573]
        //~| HELP: try using the variant's enum
        _ => {}
    }
}

fn main() {
    let myfoo = Foo::Bar;
    match myfoo {
        Foo::Bar => {}
        Foo:Bar::Baz => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: failed to resolve: use of undeclared type `Bar` 
    }
    match myfoo {
        Foo::Bar => {}
        Foo:Bar => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        //~| ERROR: cannot find type `Bar` in this scope [E0412] 
        //~| HELP: there is an enum variant `Foo::Bar`; try using the variant's enum 
    }
}

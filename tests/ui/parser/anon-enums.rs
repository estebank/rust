// Output of proposed syntax for anonymous enums from https://github.com/rust-lang/rfcs/issues/294.
// https://github.com/rust-lang/rust/issues/100741

fn foo(x: bool | i32) -> i32 | f64 {
    //~^ ERROR: anonymous enums are not supported
    //~| ERROR: anonymous enums are not supported
    match x { //~ ERROR: struct literals are not allowed here
        x: i32 => x, //~ ERROR: expected
        true => 42., //~ ERROR: expected
        false => 0.333, //~ ERROR: expected
    }
} //~ ERROR: expected

fn bar(x: (bool | i32)) -> (i32 | f64) {
    //~^ ERROR: anonymous enums are not supported
    //~| ERROR: anonymous enums are not supported
    match x { //~ ERROR: struct literals are not allowed here
        x: i32 => x, //~ ERROR: expected
        true => 42., //~ ERROR: expected
        false => 0.333, //~ ERROR: expected
    }
} //~ ERROR: expected

fn main() {
    match foo(true) {
        42: i32 => (), //~ ERROR: expected
        _: f64 => (), //~ ERROR: expected
        x: i32 => (), //~ ERROR: expected
    }
    match bar(true) {
        42: i32 => (), //~ ERROR: expected
        _: f64 => (), //~ ERROR: expected
        x: i32 => (), //~ ERROR: expected
    }
}

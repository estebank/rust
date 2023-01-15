fn foo(x: bool) -> i32 | f64 { //~ ERROR
    if x {
        42
    } else {
        0.42
    }
}

fn main() {
    match foo(true) {
        _: i32 => println!("is integer"), //~ ERROR
        _: f64 => println!("is float"), //~ ERROR
    }
}

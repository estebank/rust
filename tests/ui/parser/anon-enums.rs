fn foo(x: bool) -> i32 | f64 { //~ ERROR
    if x {
        42
    } else {
        0.42
    }
}

// fn bar(x: bool) -> i32 | f64 { //~ ERROR
//     if x {
//         42i32
//     } else {
//         0.42f64
//     }
// }


fn main() {
    // let _: i32 = foo(true);
    // let _: i32 = bar(true);
    match foo(true) {
        _: i32 => println!("is integer"), //~ ERROR
        _: f64 => println!("is float"), //~ ERROR
    }
}

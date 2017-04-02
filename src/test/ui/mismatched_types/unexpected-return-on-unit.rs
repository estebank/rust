// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn foo1() {
    return 1;
}

fn foo2() {
    2
}

fn foo3() {
    bar()
}

fn foo4() {
    if true {
        3
    } else {
        4
    }
}

fn foo6() {
    let x = Some(1);
    match x {
        Some(y) => 6,
        None => 6,
    }
}

fn foo7() {
    {
        7
    }
}

fn bar() -> usize {
    ""
}

fn main() {
    bar()
}

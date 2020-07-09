fn main() {
    let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
    //~^ ERROR expected value, found enum `Option`
}

// This case isn't currently being handled gracefully due to the macro invocation.

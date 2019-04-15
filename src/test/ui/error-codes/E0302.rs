fn main() {
    match Some(()) {
        None => { },
        option if { option = None; false } => { }, //~ ERROR E0302
        Some(_) => { }  //~^ ERROR E0384
    }
}

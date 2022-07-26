// run-pass
// check-run-results

fn main() {
    test(Some(String::from("42")));
    test(Some(String::new()));
    test(None);
}

fn test(o: Option<String>) {
    match o {
        Some("42") => println!("the answer"),
        Some(_) => println!("something else?"),
        None => println!("nil"),
    }
}
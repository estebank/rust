trait Trait {
    fn function() {}
}

impl Trait for () {}

fn main() {
    <dyn Trait>::function(); //~ ERROR the trait `Trait` is not dyn compatible
}

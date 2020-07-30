#![feature(staged_api)]
#![stable(feature = "test", since = "0")]

#[stable(feature = "test", since = "0")]
pub struct A<T>(pub T); //~ ERROR field has missing stability attribute

fn main() {
    // Make sure the field is used to fill the stability cache
    A(0).0;
}

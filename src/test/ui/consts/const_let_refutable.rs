fn main() {}

const fn slice([a, b]: &[i32]) -> i32 { //~ ERROR refutable pattern in function argument
    a + b //~ ERROR can only call other `min_const_fn` within a `min_const_fn`
}

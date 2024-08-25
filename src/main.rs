use std::fmt::Debug;

fn max<T: PartialOrd + Debug>(a: T,b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", max(1, 2));
    println!("{}", max(1.1, 1.2));
    println!("{}", max("x", "a"));
}
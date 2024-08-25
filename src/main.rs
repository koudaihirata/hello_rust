use std::{fmt::Debug, fmt::Display};

struct Point<T> {
    x: T,
    y: T,
}
impl <T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x: {:?}", self.x);
        println!("val: {}", val);
    }
}

impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}

fn main() {
    let p1 = Point{x: 2,y: 1};
    let p2 = Point{x: 2.0,y: 1.0};
    let p3 = Point{x: "a",y: "b"};
    println!("p1.max: {:?}", p1.max());
    println!("p2.max: {:?}", p2.max());
    println!("p3.max: {:?}", p3.max());

    p1.print_arg("test");
    p1.print_arg(true);

    println!("p1.min: {:?}", p1.min());
    // println!("p2.min: {:?}", p2.min()); // p2がf64の構造体であるためエラー
}
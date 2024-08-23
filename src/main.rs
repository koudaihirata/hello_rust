fn say_hello() {
    println!("Hello");
}

fn add(a: i32,b: i32) -> i32 {
    a + b
}

fn main() {
    say_hello();
    println!("{}", add(1, 2));

    let c = add(3, 4);
    // println!("{}", c);
    let d = say_hello();
    // println!("{:?}", d);
}
fn main() {
    let x = 1;
    match x {
        0 => println!("Zero!"),
        1 => {
            println!("one");
            println!("one");
        },
        _ => println!("Other"),
    };

    let y = match x {
        0 => 0,
        1 => 10,
        _ => 100
    };
    println!("{:?}", y);
}
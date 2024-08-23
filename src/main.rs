fn main() {
    println!("a");
    {
        println!("b");
    }
    println!("c");

    // シャドーイング
    let y = 10;
    println!("{}", y);
    {
        let y = 5;
        println!("{}", y);
    }
    println!("{}", y);

    let z = {
        100
    };
    println!("{}", z);
}
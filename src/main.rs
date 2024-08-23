fn main() {
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // let a = Some(1);
    // let b = Some("str");
    // let c: Option<i32> = None;

    let v = vec![1, 2, 3];
    let val = v.get(0);

    match val {
        Some(x) if *x == 1 => println!("value is 1"),
        Some(x) => println!("value exists: {}",x),
        None => println!("value is None"),
    }

    // if let Some(x) = val {
    //     println!("val={}", x);
    // }
}
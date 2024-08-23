// fn fizzbuzz(end: i32) {
//     let mut x = 1;

//     while x <= end {
//         if x % 3 == 0 && x % 5 == 0 {
//             println!("FizzBuzz");
//         } else if x % 3 == 0 {
//             println!("Fizz");
//         } else if x % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", x);
//         }
//         x += 1;
//     }
// }

// fn fizzbuzz(end: i32) {
//     let r = 1..=end;

//     for x in r {
//         match x % 15 {
//             0 => println!("FizzBuzz"),
//             3 | 6 | 9 | 12 => println!("Fizz"),
//             5 | 10 => println!("Buzz"),
//             _ => println!("{}", x)
//         }
//     }
// }

fn fizzbuzz(end: i32) {
    let r = 1..=end;

    for x in r {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", x)
        }
    }
}


fn main() {
    fizzbuzz(30);
}
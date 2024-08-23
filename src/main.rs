fn main() {
    // loop :無限ループを発生させる
    // let mut cnt = 0;
    // loop {
    //     println!("Hello");
    //     if cnt == 10 {
    //         break;
    //     }
    //     cnt += 1;
    // }

    // while
    // let mut cnt = 0;
    // while cnt <= 10 {
    //     println!("Hello");
    //     cnt += 1;
    // }

    // for
    for i in [1, 2, 3] {
        println!("Hello, {}", i);
    }

    let r = 1..=10;
    for x in r {
        println!("{}", x * x);
    }
}
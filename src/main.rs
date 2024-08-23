fn main() {
    let x = 30;

    if x > 0 {
        println!("OK!");
    }

    if x > 0 && x < 10 {
        println!("0 < x and x < 10");
    }
    if x > 0 || x < 10 {
        println!("0 < x or x < 10");
    }

    if x > 0 && x <= 10 {
        println!("first condition");
    } else if x > 11 && x <= 20 {
        println!("second condition");
    } else {
        println!("else");
    }

    let y = if x > 10 {
        x
    } else {
        "0" // 型が違うのでエラーになる
    };
}
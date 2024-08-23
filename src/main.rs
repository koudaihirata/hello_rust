fn main() {
    // 文字列
    let c1 = 'a';
    let c2 = '@';
    let c3 = '😄';

    // 文字配列
    let s1 = "Rust";

    let s2 = String::from("Python");
    let s3 = "Java".to_string();

    let mut s4 = String::from("Hello");
    s4.push_str(", Rust");
    println!("{}", s4);

    let s5 = format!("{}{}", s1, s2);
}
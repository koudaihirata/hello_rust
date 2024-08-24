mod test_module {
    pub mod test_module1 {
        pub fn test_fn1() {
            println!("Hello World1");
        }
        fn test_fn2() {
            println!("Hello Rust1");
        }
    }

    mod test_module2 {
        pub fn test_fn1() {
            println!("Hello World2");
        }
        fn test_fn2() {
            println!("Hello Rust2");
        }
    }

}

use test_module::test_module1::test_fn1;

use crate::test_module::test_module1;
fn main() {
    crate::test_module::test_module1::test_fn1();
    // crate::test_module::test_module2::test_fn1(); 上でpubを指定していないからエラーが出る

    test_fn1();
}
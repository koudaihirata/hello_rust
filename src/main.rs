mod test_module;

use test_module::*;
fn main() {
    crate::test_module::test_module1::test_fn1();
    crate::test_module::test_module2::test_fn1();

    test_module1::test_fn1();
    test_module2::test_fn1();
}
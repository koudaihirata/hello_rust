use std::rc::Rc;

fn main() {
    let x = Box::new(1);
    println!("x: {:p}", x);
    println!("x + 2 = {}", *x + 2);

    let a = Rc::new("Hello".to_string());
    println!("count1: {}", Rc::strong_count(&a));
    {
        let b = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        println!("count2: {}", Rc::strong_count(&a));
    }
    println!("count3: {}", Rc::strong_count(&a));
}
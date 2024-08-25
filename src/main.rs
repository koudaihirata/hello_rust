use hello_rust::sample_trait::{double_area, Circle, Rectangle, Shape};

fn main() {
    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle {
        radius: 2.0,
    };

    // println!("Rectangle area is: {}", rect.calc_area());
    // println!("Rectangle perimeter is: {}", rect.calc_perimeter());
    // Rectangle::do_something();
    // println!("circle area is: {}", circle.calc_area());
    // println!("circle perimeter is: {}", circle.calc_perimeter());
    // Circle::do_something();

    // println!("Rectangle default: {}", rect.default_something());
    // println!("Circle default: {}", circle.default_something());

    println!("Rectangle double area: {}", double_area(&rect));
    println!("Circle double area: {}", double_area(&circle));
}
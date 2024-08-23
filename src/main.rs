struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Rectangle {width, height}
    }
}

fn main() {
    let mut rectangle: Rectangle = Rectangle::new(10, 10);
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    println!("area: {}", rectangle.area());
}
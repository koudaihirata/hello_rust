struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rectangle: Rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    println!("area: {}", rectangle.area());
}
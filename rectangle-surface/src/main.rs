#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area (&self) -> usize {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, Rectangle!");

    let rect = Rectangle {
        width: 100,
        height: 150,
    };

    let area = rect.area();

    println!("the area of your Rectangle : {:?} is {}", rect, area)
}

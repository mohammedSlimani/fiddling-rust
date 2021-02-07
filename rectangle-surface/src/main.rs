#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area (&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    fn square(size: usize) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("Hello, Rectangle!");

    let rect = Rectangle {
        width: 100,
        height: 150,
    };

    let rect1 = Rectangle {
        width: 90,
        ..rect
    };

    let rect2 = Rectangle {
        ..rect
    };

    let rect3 = Rectangle {
        width: 111,
        height: 112
    };

    let area = rect.area();

    println!("the area of your Rectangle : {:?} is {}", rect, area);
    println!("Can {:?} contain {:?} ? {}", rect, rect1, rect.can_hold(&rect1));
    println!("Can {:?} contain {:?} ? {}", rect, rect2, rect.can_hold(&rect2));
    println!("Can {:?} contain {:?} ? {}", rect, rect3, rect.can_hold(&rect3));

    // square is a function not a method!
    let my_square = Rectangle::square(156);

    println!("Here is a square {:?} ", my_square)
}

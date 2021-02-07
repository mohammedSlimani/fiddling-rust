struct Rectangle {
    width: usize,
    hight: usize,
}

fn calculate_surface(rect: &Rectangle) -> usize {
    rect.width * rect.hight
}

fn main() {
    println!("Hello, Rectangle!");

    let rect = Rectangle {
        width: 100,
        hight: 150,
    };

    let surface = calculate_surface(&rect);

    println!("the surface of your Rectangle is : {}", surface)
}

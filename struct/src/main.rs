struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect = Rectangle{ width: 30, height: 50 };

    println!(
        "the area of the rectanble is {}.", area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    println!("the rectangle is {:?}", rect1);

    println!(
        "The area of the rectangle is: {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

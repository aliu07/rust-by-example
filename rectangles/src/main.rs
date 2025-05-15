#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Pretty print
    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        // dbg! macro takes and returns ownership
        width: dbg!(30 * scale),
        height: 50,
    };

    // Here, we don't want dbg! to take ownership, so we pass a reference
    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

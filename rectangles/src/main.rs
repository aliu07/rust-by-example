#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define methods within scope of Rectangle struct
// also called ASSOCIATED FUNCTIONS
impl Rectangle {
    // Need & in front of self since borrowing immutable
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Can have many impl blocks for same struct
impl Rectangle {
    // We can define associated functions that don’t have self
    // as their first parameter (and thus are not methods) because
    // they don’t need an instance of the type to work with.
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels (using function call).",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels (using method call).",
        rect1.area()
    );

    // Pretty print
    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        // dbg! macro takes and returns ownership
        width: dbg!(30 * scale),
        height: 60,
    };

    // Here, we don't want dbg! to take ownership, so we pass a reference
    dbg!(&rect2);

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // The :: syntax is used for both associated functions and namespaces
    // created by modules
    let sq = Rectangle::square(3);

    println!("This is a square: {sq:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#![allow(dead_code, unused_variables)]

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // Using generic function definition
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Generics in structs
    let point1 = Point { x: 1, y: 1 };
    let point2 = Point { x: 1.1, y: 1.1 };

    println!("The x value of point1 is {}", point1.get_x());
    println!("The x value of point2 is {}", point2.get_x());

    // Example below won't work due to conflicting types
    // let wont_work = Point { x: 5, y: 4.0 };
    // But this will work due to different types
    let other_point = OtherPoint { x: 1, y: 1.1 };

    // A method that uses generic types different from its struct’s definition
    let p1 = OtherPoint { x: 5, y: 10.4 };
    let p2 = OtherPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Generics in functions
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics in structs
struct Point<T> {
    x: T,
    y: T,
}

struct OtherPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> OtherPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: OtherPoint<X2, Y2>) -> OtherPoint<X1, Y2> {
        OtherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic impl block
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// Below is an impl block that will only apply to Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

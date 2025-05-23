#![allow(dead_code, unused_variables)]

fn main() {
    // Matching on literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // The y here shadows the y in the outer scope and binds to the wrapped value of x (i.e. 5)
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    } // y in the inner scope is dropped here

    println!("at the end: x = {x:?}, y = {y}");

    // Matching multiple patterns using '|' operator
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching a range
    // Example with ints
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Example with chars
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Matching with destructuring
    let p = Point { x: 0, y: 7 };
    // Vars a, b match to the values stored in x and y of the p struct
    //
    // let Point { x: a, y: b } = p;
    //
    // Shorthand syntax (when vars match field names)
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match &msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(_) => {
            println!("Change color");
        }
    }

    // Destructuring nested structs and enums
    match &msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring values
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // Ignoring remaining parts with ..
    let origin = OtherPoint { x: 0, y: 0, z: 0 };

    match origin {
        // We list the x value and then just include the .. pattern.
        // This is quicker than having to list y: _ and z: _
        OtherPoint { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // However, using .. must be unambiguous. If it is unclear which values
    // are intended for matching and which should be ignored, Rust will give us an error.
    // The following will not compile:
    //
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {second}")
    //     },
    // }

    // Extra conditions with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // We can use match guards to bypass the shadowing related to y
    // we encountered in a previous example (line 16)
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    let script = Script::Hello { id: 5 };

    match script {
        // The binding operator allows us to capture whatever value matched the range
        // while also testing that the value in 'script' matched the range pattern
        Script::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        // In this arm, we only have a range specified in the pattern. The code associated
        // with the arm doesn't have a variable that contains the actual value of the 'id'
        // field. Thus, we cannot print out the specific 'id' value that got matched since
        // we never bound it to anything
        Script::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // No range specified on 'id'. This arm will match any 'id' value.
        Script::Hello { id } => println!("Found some other id: {id}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct OtherPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Script {
    Hello { id: i32 },
}

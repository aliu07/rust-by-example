#![allow(dead_code, unused_variables)]

use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Anything between the two pipes are params for the closure

        // The closure captures an immutable reference to the self Inventory
        // instance and passes it with the code we specify to the unwrap_or_else
        // method. Functions, on the other hand, are not able to capture their
        // environment in this way.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // More closure syntax examples
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    } // This is a function

    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure with full type annotations

    // let add_one_v3 = |x| x + 1; --> Removed typed annotations
    // The example above won't compile here due to lack of context,
    // but closures rarely need mandatory type annotations.

    // let add_one_v4 = |x| x + 1; --> Removed brackets
    // Also won't compile due to lack of context for type inference.
    // Can remove brackets since closure body only has a single
    // expression

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // Uncommenting the line below will cause a compilation error.
    // let n = example_closure(5);

    // The first time we call example_closure with the String
    // value, the compiler infers the type of x and the return
    // type of the closure to be String. Those types are then
    // locked into the closure in example_closure, and we get
    // a type error when we next try to use a different type
    // with the same closure.

    // Closures can capture values from their environment in three ways,
    // which directly map to the three ways a function can take a parameter:
    // borrowing immutably, borrowing mutably, and taking ownership.

    // Capturing immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Capturing mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // Uncommenting the line below will cause a compilation error.
    // This is because we have defined a closure that captures a
    // mutable reference to our list. If we print, we are creating
    // another immutable reference to the list. Rust does not allow
    // simultaneous mutable and immutable borrows.

    // println!("Before defining closure: {list:?}");

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // Transferring ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Add move keyword before param list to transfer ownership of list to new thread

    // Here we need to specify move (an immutable borrow would not compile) since the
    // main thread might end before the new thread. With an immutable reference, we
    // cannot guarantee that the list will be valid when the new thread accesses it
    // using an immutable ref.
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

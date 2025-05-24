#![allow(dead_code)]

use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

// Def of Add trait. Note how the default type of Rhs is Self. When we implement
// this trait for Point, Self with default to Point.
//
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }
//
// You’ll use default type parameters in two main ways:
//     1. To extend a type without breaking existing code
//     2. To allow customization in specific cases most users won’t need

impl Add for Point {
    // The Add trait has an associated type named Output that determines the type
    // returned from the add() method.
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Here, we specify the value of the Rhs type parameter to Meters instead of Self which
// would give Millimeters. We define custom behaviour for the '+' operator between
// instances of Millimeters and Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let result = Millimeters(5) + Meters(1);
    println!("{result:?}");

    // Disambiguating between methods with the same name
    // By default, if many methods with same name, compiler will resort to
    // method directly implemented on the type (i.e. the one directly in the
    // Human struct)
    let person = Human;
    person.fly();
    // To call fly() defined under Pilot and Wizard, we need to be more explicit
    Pilot::fly(&person);
    Wizard::fly(&person);

    // Can't use Animal::baby_name() for explicit method call here because baby_name()
    // does not take a self param... so Rust will not know which implementation to use
    // since many types can implement the Animal trait
    //
    // We need to use fully qualified syntax (the angled brackets). This tells the
    // compiler we want to call the baby_name() method from the Animal trait as
    // implemented on Dog by saying we want to treat the Dog as an Animal.
    //
    // You only need to use this more verbose syntax (fully qualified syntax) in cases where there
    // are multiple implementations that use the same name and Rust needs help to identify which
    // implementation you want to call.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Super trait example
    let p = Point { x: 5, y: 4 };

    p.outline_print();

    // Implement Display trait on Vec<T>... both the Display trait and the Vec<T> type are
    // defined outside our create... violate orphan rule!
    //
    // But we can create a local wrapper and define the trait on said Wrapper. No runtime
    // penalty for using this pattern. Compiler elides wrapper type!
    //
    // The downside of using this technique is that Wrapper is a new type, so it doesn’t have
    // the methods of the value it’s holding. We would have to implement all the methods of
    // Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow
    // us to treat Wrapper exactly like a Vec<T>.
    //
    // If we wanted the new type to have every method the inner type has, implementing the Deref
    // trait on the Wrapper to return the inner type would be a solution.
    //
    // If we didn’t want the Wrapper type to have all the methods of the inner type—for example,
    // to restrict the Wrapper type’s behavior—we would have to implement just the methods we do
    // want manually.
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

// Multiple methods with same name...
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// More examples with methods bearing same name...
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// SUPER TRAITS
//
// Sometimes you might write a trait definition that depends on another trait: for a type
// to implement the first trait, you want to require that type to also implement the second
// trait. You would do this so that your trait definition can make use of the associated
// items of the second trait. The trait your trait definition is relying on is called a
// supertrait of your trait.

// Here, we specify OutlinePrint requires the Display trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Using the newtype pattern to implement external traits on external types
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

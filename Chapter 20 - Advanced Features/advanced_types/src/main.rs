#![allow(unused_variables)]

fn main() {
    // By using type aliases, we don't get the type-checking benefits that we would
    // from using the newtype pattern. The main use case for aliases is to reduce
    // repetition especially when we have lengthy types like:
    //
    // Box<dyn Fn() + Send + 'static>
    type_alias_example();
}

// This is a type alias
type Kilometers = i32;

fn type_alias_example() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    println!("====================================");

    println!("Guess is: {}", take_guess("5"));
}

// Rust has a special type named ! that’s known in type theory lingo as the empty
// type because it has no values. We prefer to call it the never type because it
// stands in the place of the return type when a function will never return (also
// called diverging functions). Here is an example:
fn take_guess(guess: &str) -> u32 {
    // All arms in the match block need to return the same type. The first branch
    // returns a u32. You might wonder what the second branch with 'continue' returns...
    // 'continue' is a ! value; it never returns! So the compiler infers the
    // match block to return u32. Formally, ! can be coerced into any type.
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }

    // 'loop' blocks are also of the ! type since they never return. However,
    // if we add a break statement, the type will not be ! anymore.
}

// To work with dynamically sized types (DSTs), Rust provides the Sized trait
// to determine whether or not a type’s size is known at compile time. This
// trait is automatically implemented for everything whose size is known at
// compile time. In addition, Rust implicitly adds a bound on Sized to every
// generic function. That is, a generic function definition like this:
// fn generic<T>(t: T) {
//     // --snip--
// }
//
// is actually treated as though we had written this:
//
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }
//
// By default, generic functions will work only on types that have a known size
// at compile time. However, you can use the following special syntax to relax
// this restriction:
//
// fn generic<T: ?Sized>(t: &T) {
// --snip--
// }
//
// A trait bound on ?Sized means “T may or may not be Sized” and this notation
// overrides the default that generic types must have a known size at compile time.
// The ?Trait syntax with this meaning is only available for Sized, not any other
// traits.

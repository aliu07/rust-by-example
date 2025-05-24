#![allow(dead_code)]

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    closure_vs_function_example();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn(i32) specifies that f is a function that takes one parameter of type i32
// and returns i32.
//
// fn with a lowercare f is not to be confused with the Fn trait for closures.
// The fn type implements all three of the closure traits (Fn, FnMut, and FnOnce).
//
// One example of where you would want to only accept fn and not closures is when
// interfacing with external code that doesn’t have closures: C functions can accept
// functions as arguments, but C doesn’t have closures.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn closure_vs_function_example() {
    let list_of_numbers = vec![1, 2, 3];

    let _list_of_strings_using_closure: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    let _list_of_strings_using_function: Vec<String> =
        // Note that we must use the fully qualified syntax that we talked about in
        // “Advanced Traits” because there are multiple functions available named to_string.
        list_of_numbers.iter().map(ToString::to_string).collect();

    // Recall from “Enum values” in Chapter 6 that the name of each enum variant that we define
    // also becomes an initializer function. We can use these initializer functions as function
    // pointers that implement the closure traits, which means we can specify the initializer
    // functions as arguments for methods that take closures
    enum Status {
        Value(u32),
        Stop,
    }

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn returning_a_closure_example() {
    // Here we have two functions, returns_closure and returns_initialized_closure,
    // which both return impl Fn(i32) -> i32. Notice that he closures that they return
    // are different, even though they implement the same type. If we try to compile
    // this, Rust lets us know that it won’t work. Instead, we solve this issue by wrapping
    // each closure behind a pointer, a Box in this case!
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + init)
    }

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

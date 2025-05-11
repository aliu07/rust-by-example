fn main() {
    println!("Hello, world!");

    another_function(5);

    another_different_function();
}

// This is another function
fn another_function(x: i32) {
    println!("Another function. The value of x is: {x}");
}

// This is another different function
fn another_different_function() {
    another_function(5)
}

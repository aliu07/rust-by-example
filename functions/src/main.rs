fn main() {
    println!("Hello, world!");

    another_function(5);

    another_different_function();
}

fn another_function(x: i32) {
    println!("Another function. The value of x is: {x}");
}

fn another_different_function() {
    another_function(5)
}

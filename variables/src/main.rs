fn main() {
    // Mutable var declaration
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing example
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Floats
    let z = 2.0; // Defaults to f64
    println!("The value of z is: {z}");
    let w: f32 = 3.0;
    println!("The value of w is: {w}");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{tup:#?}");
    // Destructuring a tuple
    let (_, b, _) = tup;
    println!("The value of b is : {b}");
    // Fetching using period (.)
    let five_hundred = tup.0;
    println!("The value of five_hundred is : {five_hundred}");
}

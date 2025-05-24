use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    // Declarative macro example with custom vec! implementation
    let v = my_vec!([1, 2, 3]);
    println!("{:?}", v);

    // Procedural derive macro
    Pancakes::hello_macro();

    // Attribute-like macro example (for structs and enum)
}

// Declarative macros with macro_rules!
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

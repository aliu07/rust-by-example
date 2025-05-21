#![allow(dead_code)]

mod cons_list;
mod my_box;

use my_box::MyBox;

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let list = cons_list::const_list_example();
    println!("{list:?}");

    // References and dereferencing (* operator)
    let x = 5;
    let y = &x; // y is a reference
    let z = Box::new(x); // z is a box instance pointing to a copied value of x

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // need to unwrap or dereference
    assert_eq!(*z, 5); // also need to unwrap since z is a smart ptr

    // Defining our own smart pointer
    let a = MyBox::new(5);

    assert_eq!(*a, 5); // we are able to deref our MyBox instance
    // Behind the scenes, Rust actually ran the following code:
    // *(a.deref())
    // Rust substitutes the * operator with a call to the deref method
    // and then a plain dereference so we don’t have to think about whether
    // or not we need to call the deref method. This Rust feature lets us
    // write code that functions identically whether we have a regular
    // reference or a type that implements Deref.

    // We can call the hello function with a string slice as an argument,
    // such as hello("Rust");, for example. Deref coercion makes it possible
    // to call hello with a reference to a value of type MyBox<String>
    hello("Rust");
    let m = MyBox::new("Rust");
    hello(&m);

    // Without deref coercion, we would have needed to write
    hello(&(*m)[..]);
    // The (*m) dereferences the MyBox<String> into a String. Then the &
    // and [..] take a string slice of the String that is equal to the whole
    // string to match the signature of hello. This code without deref
    // coercions is harder to read, write, and understand with all of these
    // symbols involved. Deref coercion allows Rust to handle these conversions
    // for us automatically.
}

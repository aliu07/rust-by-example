#![allow(dead_code)]

mod cons_list;
mod cons_list_rc;
mod custom_smart_ptr;
mod my_box;

use custom_smart_ptr::CustomSmartPointer;
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

    // Example for drop trait
    {
        let _c = CustomSmartPointer {
            data: String::from("some stuff"),
        };

        // _c.drop();
        // Manual drops are not allowed! If we manually drop here, the compiler
        // would also drop when _c goes out of scope. This would cause a double
        // free error!

        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
    } // _c and _d go out of scope here, should see a println
    // Notice the order of drops, _d is dropped before _c
    // Dropping goes in reverse order of creation!

    {
        // Instead of manually calling the method drop (from the Drop trait),
        // we can call a built-in force-drop function included in the prelude.
        // This method is technically under the std::mem::drop namespace.
        let n = CustomSmartPointer {
            data: String::from("more data"),
        };

        // This method still calls the code in the drop method of CustomSmartPointer,
        // so we will see a println.
        drop(n);

        println!("CustomSmartPointer dropped before end of scope!");
    }

    // Rc<T> examples
    cons_list_rc::cons_list_rc_example();
}

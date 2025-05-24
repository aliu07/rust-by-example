#![allow(dead_code)]

// To switch to unsafe Rust, use the unsafe keyword and then start a new block
// that holds the unsafe code. You can take five actions in unsafe Rust that
// you can’t in safe Rust, which we call unsafe superpowers. Those superpowers
// include the ability to:
// - Dereference a raw pointer
// - Call an unsafe function or method
// - Access or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of a union

use std::slice;

// Static vars can only store refs with the 'static lifetime
// Declare static vars in SCREAMING_SNAKE_CASE
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    unsafe_ptrs_example();

    // Calling an unsafe function
    unsafe {
        dangerous();
    }

    creating_safe_abstraction_over_unsafe_code_example();

    // static var examples
    println!("name is: {HELLO_WORLD}");

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        // The compiler will not allow you to create references to a mutable static variable.
        // You can only access it via a raw pointer, created with one of the raw borrow operators.
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

// Different from references and smart pointers, raw pointers:
// - Are allowed to ignore the borrowing rules by having both immutable and
//   mutable pointers or multiple mutable pointers to the same location
// - Aren’t guaranteed to point to valid memory
// - Are allowed to be null
// - Don’t implement any automatic cleanup
fn unsafe_ptrs_example() {
    let mut num = 5;

    // Notice that we don’t include the unsafe keyword in this code.
    // We can create raw pointers in safe code; we just can’t dereference
    // raw pointers outside an unsafe block, as you’ll see in a bit.
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // We create a raw pointer whose validity we can’t be so certain of,
    // using as to cast a value instead of using the raw borrow operators.
    let address = 0x012345usize;
    let _r = address as *const i32;
}

// Defining an unsafe function
unsafe fn dangerous() {}

// We will try manually implementing split_at_mut which splits a vector into 2 slices given
// an index and returns each half.
fn creating_safe_abstraction_over_unsafe_code_example() {
    fn custom_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        // The code below will not compile...
        // Rust’s borrow checker can’t understand that we’re borrowing different parts
        // of the slice; it only knows that we’re borrowing from the same slice twice.
        // Borrowing different parts of a slice is fundamentally okay because the two
        // slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
        // know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
        //
        // (&mut values[..mid], &mut values[mid..])
        //
        // Instead, we need to use an unsafe block
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // Notice how this function call is safe (i.e. we don't need the unsafe keyword
    // even though the function itself contains unsafe code)
    let (a, b) = custom_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// Whenever we perform an unsafe operation, it is idiomatic to write a comment
// starting with SAFETY to explain how the safety rules are upheld.
//
/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

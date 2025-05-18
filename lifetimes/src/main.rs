#![allow(dead_code, unused_variables)]

use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // Lifetimes with string slices
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Lifetimes with structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    // The struct created cannot outlive the reference it holds in its
    // parts field
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static lifetime is special, affected reference can live for the
    // entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
}

// We want the signature to express the following constraint: the
// returned reference will be valid as long as both the parameters
// are valid. This is the relationship between lifetimes of the
// parameters and the return value. We’ll name the lifetime 'a and
// then add it to each reference.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Syntax of generics, traits, and lifetimes all together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

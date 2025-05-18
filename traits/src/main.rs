#![allow(unused_variables, dead_code)]

use std::fmt::{Debug, Display};

fn main() {
    let post = returns_summarizable();

    println!("1 new social post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// Declare trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Function with default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Returning a trait type
// Condition -> make sure that return type is consistent, the function would
// not compile if it returned either a SocialPost or a NewsArticle for ex
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// Traits as function parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Allows 2 diff types both implementing the Summary trait
pub fn example(item1: &impl Summary, item2: &impl Summary) {}
// Requires item1 and item2 to be of same type and that type
// needs to implement the Summary trait. This is an example
// of trait bound on generic types.
pub fn other_example<T: Summary>(item1: &T, item2: &T) {}
// Can implement many traits
pub fn compound_example(item: &(impl Summary + Display)) {}
pub fn other_compound_example<T: Summary + Display>(item: &T) {}

// For functions with many generic types, using trait bounds can
// make the function signature harder to read. We can use where
// clauses to be clearer.
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    return 0;
}
// Turns into...
fn some_function_clearer<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}

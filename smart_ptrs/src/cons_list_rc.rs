// See Figure 15-3: https://doc.rust-lang.org/book/ch15-04-rc.html#using-rct-to-share-data

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::cons_list_rc::List::{Cons, Nil};
// Via immutable references, Rc<T> allows you to share data between multiple parts of your
// program for reading only. If Rc<T> allowed you to have multiple mutable references too,
// you might violate one of the borrowing rules. Multiple mutable borrows to the same place
// can cause data races and inconsistencies.
use std::rc::Rc;

pub fn cons_list_rc_example() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::clone(&a) here doesn't make a deep copy, it only increments the ref count
    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    // Add inner scope
    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

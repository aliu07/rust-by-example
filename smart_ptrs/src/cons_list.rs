#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::cons_list::List::{Cons, Nil};

pub fn const_list_example() -> List {
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
}

#[derive(Debug)]
pub enum List {
    // Allows multiple owners + mutability now!
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::cons_list_rc_with_refcell::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn cons_list_rc_with_refcell_example() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

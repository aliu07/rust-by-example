use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // We want a Node to own its children, and we want to share that
    // ownership with variables so we can access each Node in the tree
    // directly. To do this, we define the Vec<T> items to be values
    // of type Rc<Node>. We also want to modify which nodes are children
    // of another node, so we have a RefCell<T> in children around the
    // Vec<Rc<Node>>.
    children: RefCell<Vec<Rc<Node>>>,
    // To make the child node aware of its parent, we need to add a parent
    // field to our Node struct definition. The trouble is in deciding what
    // the type of parent should be. We know it can’t contain an Rc<T> because
    // that would create a reference cycle with leaf.parent pointing to branch
    // and branch.children pointing to leaf, which would cause their strong_count
    // values to never be 0.
    //
    // Thinking about the relationships another way, a parent node should own
    // its children: if a parent node is dropped, its child nodes should be dropped
    // as well. However, a child should not own its parent: if we drop a child node,
    // the parent should still exist. This is a case for weak references!
    //
    // So, instead of Rc<T>, we’ll make the type of parent use Weak<T>,
    // specifically a RefCell<Weak<Node>>.
    parent: RefCell<Weak<Node>>,
}

pub fn tree_example() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

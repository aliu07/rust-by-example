// Instead of explict inheritance, we can mimic same concept through
// traits to abstract common behaviour.
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Need dyn keyword here because we don't know how big each element implementing
    // the Draw trait will be at compile time!
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// You might think we could have used trait bounds (generics with trait constraints),
// but we will see below its limitations and why it might not work for our use case:
//
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
//
// This implementation restricts us to a Screen instance that has a list of
// components all of a singular type instead of being able to contain multiple
// different types.

// Onto defining different types of components...
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("drawing a button...");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("drawing a select box...");
    }
}

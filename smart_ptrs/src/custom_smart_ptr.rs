pub struct CustomSmartPointer {
    pub data: String,
}

// Implementing the Drop trait tells the compiler what code to
// run when our smart pointer goes out of scope
impl Drop for CustomSmartPointer {
    // Requires a mutable reference to self
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

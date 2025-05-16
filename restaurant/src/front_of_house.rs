// Need to add pub to module too since private by default
// Module is only container. We also need to make its contents
// i.e. specific functions public if we want to access them
// from other modules
pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

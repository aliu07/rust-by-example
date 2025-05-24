fn main() {
    // Since we declared abs() to be safe, we do not need to wrap
    // the call within an unsafe block.
    println!("Absolute value of -3 according to C: {}", abs(-3));
}

// Interating with the abs function from the C std lib
// Every block declared within an unsafe extern block is implicitly unsafe.
// However, some foreign function interface (FFI) functions are safe to call.
// The abs() function does not have any memory safety risks, so we need to
// specify that it is safe within the unsafe extern block.
unsafe extern "C" {
    // Marking a function as safe does not inherently make it safe! Instead,
    // it is like a promise you are making to Rust that it is safe. It is
    // still your responsibility to make sure that promise is kept!
    safe fn abs(input: i32) -> i32;
}

// We can also export Rust functions to be called from other languages
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

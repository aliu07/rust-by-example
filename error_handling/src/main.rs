#![allow(dead_code, unused_variables)]

mod recoverable;
mod unrecoverable;

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // unrecoverable::panic();
    // unrecoverable::out_of_bounds();

    // recoverable::open_file();

    let greeting_file = File::open("doesNotExistFile.txt")?;

    Ok(())
}

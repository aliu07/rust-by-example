use art::kinds::PrimaryColor;
use art::utils::mix;
use std::process;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    println!(
        "{:?}",
        mix(red, yellow).unwrap_or_else(|err| {
            eprintln!("[ERROR] {err}");
            process::exit(1);
        })
    );
}

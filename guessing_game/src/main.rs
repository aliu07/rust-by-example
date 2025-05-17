use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Create custom struct to avoid having to perform
// validation on value (ensuring in [1, 100] range)
// in every single function. If instead we receive
// a Guess struct, we know for a fact the value is
// valid with how the new() function is defined.
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The number has to be between 1 and 100.");
        }

        Guess { value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.get_value());

        match guess.get_value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

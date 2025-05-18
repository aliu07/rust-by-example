#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// From our guessing game. This custom struct was created to ensure
// that the guess value was within range of 1-100.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // Bring everything visible to parent module into scope
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Writing without macro and using Result as return type
    #[test]
    fn it_works_variant() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Each test ran in new thread. Panic kills thread which
    // makes test fail.
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Adding a custom error message in case of failure
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was '{result}'"
        );
    }

    #[test]
    // Test can panic for all sorts of reasons, so best to include
    // an expected value with the should_panic annotation. The
    // expected value is a substring we expect in the panic message.
    // What you choose to specify depends on how much of the panic
    // message is unique or dynamic and how precise you want your
    // test to be.
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

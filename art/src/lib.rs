//! # Art
//!
//! A library for modeling artistic concepts.

// Re-export statements
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, &'static str> {
        if c1 == c2 {
            return Err("same primary color provided twice");
        }

        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
                Ok(SecondaryColor::Purple)
            }
            (PrimaryColor::Red, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Red) => Ok(SecondaryColor::Orange),
            (PrimaryColor::Blue, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Blue) => Ok(SecondaryColor::Green),
            _ => Err("not a valid combination"),
        }
    }
}

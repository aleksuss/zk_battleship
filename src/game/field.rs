use std::fmt;

/// Declares a field size for the game
pub const FIELD_SIZE: usize = 10;

/// Fixed size matrix that represents the game field
#[derive(Clone, PartialEq, Copy)]
pub struct Battlefield(pub [CellType; FIELD_SIZE * FIELD_SIZE]);

/// Represents coordinates inside the field
#[derive(Clone, PartialEq, Copy)]
pub struct XY(pub usize, pub usize);

/// A type of the cell in the game field
#[derive(Clone, PartialEq, Copy)]
pub enum CellType {
    EMPTY,
    OCCUPIED,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EMPTY => "🌊",
                Self::OCCUPIED => "🛳️",
            }
        )
    }
}
use std::fmt;
use std::ops::{Index, IndexMut};

use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng as SmallRng;
// small_rng is not enabled on the Playground
use rand::seq::SliceRandom;

use crate::ship::{Ship, ShipShape};

pub const FIELD_SIZE: usize = 10;
#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 9] = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];

#[derive(Clone, PartialEq, Copy)]
pub struct XY(pub usize, pub usize);

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

pub struct Battlefield {
    field: [CellType; FIELD_SIZE * FIELD_SIZE],
    hits: Vec<XY>,
}

impl fmt::Display for Battlefield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, element) in self.field.iter().enumerate() {
            let xy = XY(index % FIELD_SIZE, index / FIELD_SIZE);

            // Start of line
            if xy.0 == 0 {
                writeln!(f)?;
            }

            if self.hits.contains(&xy) { write!(f, "{}", element)?; } else { write!(f, "⬜️")?; }
        }
        Ok(())
    }
}

impl Index<XY> for Battlefield {
    type Output = CellType;

    fn index(&self, xy: XY) -> &CellType {
        &self.field[xy.0 + xy.1 * FIELD_SIZE]
    }
}

impl IndexMut<XY> for Battlefield {
    fn index_mut(&mut self, xy: XY) -> &mut CellType {
        &mut self.field[xy.0 + xy.1 * FIELD_SIZE]
    }
}

impl Battlefield {
    fn can_place_ship(&self, ship: Ship) -> bool {
        // I. Construct a bounding box for the placed ship.
        let bounds = 0..(FIELD_SIZE as isize);
        for xy in ship {
            // Move in every box direction.
            for direction in &DIRECTIONS {
                // Indices cannot be negative or >= FIELD_SIZE.
                if !bounds.contains(&(xy.0 as isize + direction.0))
                    || !bounds.contains(&(xy.1 as isize + direction.1))
                {
                    continue;
                }
                let bounding_box_cell = self[XY(
                    (xy.0 as isize + direction.0) as usize,
                    (xy.1 as isize + direction.1) as usize,
                )];
                // If there's a ship within a bounding box, halt the loop -- we cannot place the ship here.
                if bounding_box_cell == CellType::OCCUPIED {
                    return false;
                }
            }
        }

        // II. Check whether the cells that are being used to place the ship onto are occupied.
        let bounds = 0..FIELD_SIZE;
        for xy in ship {
            if !bounds.contains(&xy.0) || !bounds.contains(&xy.1) {
                return false;
            }
            let current_cell = self[xy];
            if let CellType::OCCUPIED = current_cell {
                return false;
            }
        }
        true
    }

    fn get_available_cells(&self, shape: ShipShape) -> Vec<XY> {
        (0..FIELD_SIZE)
            .cartesian_product(0..FIELD_SIZE)
            .map(|(x, y)| XY(x, y))
            .filter(|&xy| self.can_place_ship(Ship { xy, shape }))
            .collect()
    }

    fn emplace_ships(&mut self, size: usize, rng: &mut impl Rng) {
        // Flip a coin to determine an alignment (horizontal / vertical).
        let dxy = if rng.gen() { XY(1, 0) } else { XY(0, 1) };
        let shape = ShipShape { dxy, size };
        // Get the vector of appropriate cells.
        let cell_coordinates = self.get_available_cells(shape);
        let xy = *cell_coordinates.choose(rng).unwrap();
        let ship = Ship { xy, shape };
        // Place a ship!
        for xy in ship {
            self[xy] = CellType::OCCUPIED;
        }
    }

    pub fn fire(&mut self, value: XY) {
        self.hits.push(value)
    }

    pub fn generate() -> Self {
        /* Generating the field. */
        let mut result = Self {
            field: [CellType::EMPTY; FIELD_SIZE * FIELD_SIZE],
            hits: Vec::new(),
        };
        let mut rng: SmallRng = SmallRng::from_entropy();
        for ship_size in &[4, 3, 3, 2, 2, 2, 1, 1, 1, 1] {
            result.emplace_ships(*ship_size, &mut rng);
        }
        result
    }
}

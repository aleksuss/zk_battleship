pub use crate::battleship::XY;

#[derive(Copy, Clone)]
pub struct Ship {
    pub xy: XY,
    pub shape: ShipShape,
}

#[derive(Copy, Clone)]
pub struct ShipShape {
    pub dxy: XY,
    pub size: u8,
}

#[derive(Copy, Clone)]
pub struct ShipType {
    // the size of the ship, value in range [1..4]
    pub ship_size: u8,
    // count of ships of this type in the field
    pub count: u8,
}

#[allow(clippy::copy_iterator)]
impl Iterator for Ship {
    type Item = XY;

    fn next(&mut self) -> Option<XY> {
        if self.shape.size > 0 {
            let result = self.xy;
            self.xy.0 += self.shape.dxy.0;
            self.xy.1 += self.shape.dxy.1;
            self.shape.size -= 1;
            Some(result)
        } else {
            None
        }
    }
}

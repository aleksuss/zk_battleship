pub use crate::battlefield::XY;

#[derive(Copy, Clone)]
pub struct Ship {
    pub xy: XY,
    pub shape: ShipShape,
}

#[derive(Copy, Clone)]
pub struct ShipShape {
    pub dxy: XY,
    pub size: usize,
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

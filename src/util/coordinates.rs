use winit::dpi::PhysicalPosition;

use std::ops::{Add, Sub};

#[derive(Default, Copy, Clone, Debug)]
pub struct Coordinates(f64, f64);

impl Coordinates {
    pub fn to_physical(&self) -> PhysicalPosition<f64> {
        PhysicalPosition::new(self.0, self.1)
    }
}

impl From<&PhysicalPosition<f64>> for Coordinates {
    fn from(value: &PhysicalPosition<f64>) -> Self {
        Coordinates(value.x, value.y)
    }
}

impl From<&PhysicalPosition<i32>> for Coordinates {
    fn from(value: &PhysicalPosition<i32>) -> Self {
        let x = value.x as f64;
        let y = value.y as f64;

        Coordinates(x, y)
    }
}

impl Add<Self> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x = self.0 + rhs.0;
        let new_y = self.1 + rhs.1;

        Coordinates(new_x, new_y)
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_x = self.0 - rhs.0;
        let new_y = self.1 - rhs.1;

        Coordinates(new_x, new_y)
    }
}

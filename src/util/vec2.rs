use winit::dpi::{PhysicalPosition, PhysicalSize};

use std::ops::{Add, Sub};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2(f64, f64);

impl Vec2 {
    pub fn to_physical_position(&self) -> PhysicalPosition<f64> {
        PhysicalPosition::new(self.0, self.1)
    }

    pub fn half(&self) -> Self {
        Vec2(self.0 / 2.0, self.1 / 2.0)
    }
}

impl From<&PhysicalPosition<f64>> for Vec2 {
    fn from(value: &PhysicalPosition<f64>) -> Self {
        Vec2(value.x, value.y)
    }
}

impl From<&PhysicalPosition<i32>> for Vec2 {
    fn from(value: &PhysicalPosition<i32>) -> Self {
        let x = value.x as f64;
        let y = value.y as f64;

        Vec2(x, y)
    }
}

impl From<&PhysicalSize<u32>> for Vec2 {
    fn from(value: &PhysicalSize<u32>) -> Self {
        let w = value.width as f64;
        let h = value.height as f64;

        Vec2(w, h)
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x = self.0 + rhs.0;
        let new_y = self.1 + rhs.1;

        Vec2(new_x, new_y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_x = self.0 - rhs.0;
        let new_y = self.1 - rhs.1;

        Vec2(new_x, new_y)
    }
}

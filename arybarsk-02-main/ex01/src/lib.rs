pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
pub fn new(x: f32, y: f32) -> Self
{
    Self {x, y}
}

pub fn zero() -> Self
{
    Self {x: 0.0, y: 0.0}
}

pub fn distance(&self, other: &Self) -> f32
{
    f32::sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
}

pub fn translate(&mut self, dx: f32, dy: f32)
{
    self.x += dx;
    self.y += dy;
}
}
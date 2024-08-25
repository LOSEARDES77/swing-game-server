use crate::space::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pos: Vec2,
    size: Vec2,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(1.0, 1.0),
        }
    }
}

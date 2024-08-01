#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pos: (f64, f64),
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Self { pos: (x, y) }
    }
}

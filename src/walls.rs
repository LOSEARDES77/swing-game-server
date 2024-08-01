#[derive(Debug, Clone, Copy)]
pub struct Wall {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Wall {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

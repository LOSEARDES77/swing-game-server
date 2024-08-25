#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
	pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}
}

impl From<[u16; 2]> for Vec2 {
    fn from(value: [u16; 2]) -> Self {
        Self { x: value[0] as f64, y: value[1] as f64 }
    }
}




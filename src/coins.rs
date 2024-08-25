#[derive(Debug, Clone, Copy)]
pub struct Coin {
    pos: (f64, f64),
	value: i32,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
			pos: (0.0, 0.0),	// MAYBE set default to a random coordenate
			value: 1 }
    }
}

impl Coin {
    pub fn new(x: f64, y: f64) -> Self {
		Self {
			pos: (x, y),
			..Default::default()
		}
	}
}



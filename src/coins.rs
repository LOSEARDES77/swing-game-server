use crate::space::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Coin {
    pos: Vec2,
	size: Vec2,
	value: i32,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
			pos: Vec2::new(0.0, 0.0),	// MAYBE set default to a random coordenate
			size: Vec2::new(1.0, 1.0),	// TODO Set the default size of the coins
			value: 1 }
    }
}

impl Coin {
    pub fn new(x: f64, y: f64) -> Self {
		Self {
			pos: Vec2::new(x, y),
			..Default::default()
		}
	}
}



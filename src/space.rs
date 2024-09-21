#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
	pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	pub fn sub(a: Vec2, b: Vec2) -> Vec2 {
		Self {
			x: a.x - b.x,
			y: a.y - b.y,
		}
	}

	pub fn add(a: Vec2, b: Vec2) -> Vec2 {
		Self {
			x: a.x + b.x,
			y: a.y + b.y,
		}
	}

	pub fn abs(v: Vec2) -> Vec2 {
		Self { x: v.x.abs(), y: v.y.abs() }
	}
}

impl From<[u16; 2]> for Vec2 {
    fn from(value: [u16; 2]) -> Self {
        Self { x: value[0] as f64, y: value[1] as f64 }
    }
}

pub fn point_inside_rect(rect_pos: Vec2, rect_size: Vec2, point: Vec2) -> bool {
	let rect_top_right = Vec2::add(rect_pos, rect_size);

	point.x >= rect_pos.x && point.x <= rect_top_right.x &&
		point.y >= rect_pos.y && point.y <= rect_top_right.y

}

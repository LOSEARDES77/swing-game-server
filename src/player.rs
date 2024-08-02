use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    color: (u8, u8, u8), // RGB color (id)
    pos: (f64, f64),     // Position (x, y)
    health: u8,          // Health
    ready: bool,         // Ready to start the match
}

impl Player {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Self {
            color,
            pos: gen_random_pos(),
            health: 100,
            ready: false,
        }
    }

    pub fn set_ready(&mut self) {
        self.ready = !self.ready;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }
}

fn gen_random_pos() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0..1900.0), rng.gen_range(0.0..1060.0))
}

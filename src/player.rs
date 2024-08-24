use rand::Rng;

use crate::handler::ClientHandler;

#[derive(Debug, Clone)]
pub struct Player {
    color: (u8, u8, u8), // RGB color (id)
    pos: (f32, f32),     // Position (x, y)
    health: u8,          // Health
    ready: bool,         // Ready to start the match
    handler: ClientHandler,
}

impl Player {
    pub fn new(color: (u8, u8, u8), handler: ClientHandler) -> Self {
        Self {
            color,
            pos: gen_random_pos(),
            health: 100,
            ready: false,
            handler,
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

    pub fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn set_health(&mut self, health: u8) {
        self.health = health;
    }

    pub fn get_handler(&self) -> &ClientHandler {
        &self.handler
    }

    pub fn get_mut_handler(&mut self) -> &mut ClientHandler {
        &mut self.handler
    }
}

fn gen_random_pos() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0..1900.0), rng.gen_range(0.0..1060.0))
}

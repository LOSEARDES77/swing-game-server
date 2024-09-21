use std::collections::HashMap;
use std::net::SocketAddr;

use rand::Rng;

use crate::game_field::{GameField, GameFieldGenerator};
use crate::player::Player;
use crate::space::Vec2;


pub const GAME_FIELD_SIZE: [u16; 2] = [1900, 1060];

#[derive(Debug, Clone)]
pub struct GameData {
    players: HashMap<SocketAddr, Player>,
	field: GameField,
    player_capacity: u8,
    match_status: bool, // Started or not
}

impl GameData {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
			field: GameField::new(GameFieldGenerator::default()),
            player_capacity: num_cpus::get() as u8,
            match_status: false,
        }
    }

    pub fn add_player(&mut self, socket_address: SocketAddr, player: Player) {
        self.players.insert(socket_address, player);
    }

    pub fn get_players(&self) -> Vec<&Player> {
        self.players.values().collect()
    }

    pub fn get_player(&self, socket_address: &SocketAddr) -> Option<&Player> {
        self.players.get(socket_address)
    }

    pub fn has_match_started(&self) -> bool {
        self.match_status
    }

    pub fn start_match(&mut self) {
        self.match_status = true;
		self.field.init();
    }

	pub fn do_tick(&mut self, dt: f32) {
		self.field.do_tick(dt);
	}
}

pub fn get_random_position(max: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();

	let x = rng.gen_range(0.0..=max.x);
	let y = rng.gen_range(0.0..=max.y);

	Vec2::new(x, y)
}

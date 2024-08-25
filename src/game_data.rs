use std::collections::HashMap;
use std::net::SocketAddr;

use rand::Rng;

use crate::enemies::Enemy;
use crate::player::Player;
use crate::space::Vec2;
use crate::walls::Wall;
use crate::coins::Coin;

pub const GAME_FIELD_SIZE: [u16; 2] = [1900, 1060];

#[derive(Debug, Clone)]
pub struct GameData {
    players: HashMap<SocketAddr, Player>,
    enemies: Vec<Enemy>,
	coins: Vec<Coin>,
    walls: Vec<Wall>,
    player_capacity: u8,
    match_status: bool, // Started or not
}

impl GameData {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            enemies: Vec::new(),
			coins: Vec::new(),
            walls: Vec::new(),
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

    pub fn add_enemy(&mut self, enemy: Enemy) {
        self.enemies.push(enemy);
    }

    pub fn get_enemies(&self) -> &Vec<Enemy> {
        &self.enemies
    }

    pub fn add_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }

    pub fn get_walls(&self) -> &Vec<Wall> {
        &self.walls
    }

    pub fn get_player(&self, socket_address: &SocketAddr) -> Option<&Player> {
        self.players.get(socket_address)
    }

    pub fn has_match_started(&self) -> bool {
        self.match_status
    }

    pub fn start_match(&mut self) {
        self.match_status = true;
    }
}

pub fn get_random_position(max: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();

	let x = rng.gen_range(0.0..=max.x);
	let y = rng.gen_range(0.0..=max.y);

	Vec2::new(x, y)
}

use crate::{
    coins::Coin,
    enemies::Enemy,
    game_data::{get_random_position, GAME_FIELD_SIZE},
    walls::Wall,
};

#[derive(Debug, Clone, Copy)]
pub struct GameFieldGenerator {
    max_enemies: u16,
    enemies_gen_rate: f32,
    max_coins: u16,
    coins_gen_rate: f32,
}

#[derive(Debug, Clone)]
pub struct GameField {
    settings: GameFieldGenerator,
    enemies: Vec<Enemy>,
    enemies_gen_counter: f32,
    coins: Vec<Coin>,
    coins_gen_counter: f32,
    walls: Vec<Wall>,
}

impl GameField {
    pub fn new(settings: GameFieldGenerator) -> Self {
        Self {
            settings,
            enemies: Vec::new(),
            enemies_gen_counter: 0.0,
            coins: Vec::new(),
            coins_gen_counter: 0.0,
            walls: Vec::new(),
        }
    }

    fn generate_coin(&self) -> Coin {
        let pos = get_random_position(GAME_FIELD_SIZE.into());

        Coin::new(pos.x, pos.y) // FIXME Does not check for collision
    }

    fn generate_enemy(&self) -> Enemy {
        let pos = get_random_position(GAME_FIELD_SIZE.into());

        Enemy::new(pos.x, pos.y) // FIXME Does not check for collision
    }
}

impl Default for GameFieldGenerator {
    fn default() -> Self {
        Self {
            max_enemies: 20,
            enemies_gen_rate: 1.0,
            max_coins: 5,
            coins_gen_rate: 5.0,
        }
    }
}

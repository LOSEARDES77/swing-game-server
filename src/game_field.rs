use crate::{coins::Coin, enemies::Enemy, game_data::{get_random_position, GAME_FIELD_SIZE}};

pub struct GameFieldGeneratorSettings {
    max_enemies: u16,
	enemies_gen_rate: f32,
	max_coins: u16,
	coins_gen_rate: f32,
}

struct GameFieldGenerator {
    settings: GameFieldGeneratorSettings,
	current_enemies: u16,
	enemies_gen_counter: f32,
	current_coins: u16,
	coins_gen_counter: f32,
}

impl GameFieldGenerator {
    pub fn new(settings: GameFieldGeneratorSettings) -> Self {
		Self {
			settings,
			current_enemies: 0,
			enemies_gen_counter: 0.0,
			current_coins: 0,
			coins_gen_counter: 0.0,
		}
	}

	fn generate_coin(&self) -> Coin {
		let pos = get_random_position(GAME_FIELD_SIZE[0] as f64, GAME_FIELD_SIZE[1] as f64);

		Coin::new(pos.0, pos.1)	// FIXME Does not check for collision 
	}

	fn generate_enemy(&self)  -> Enemy {
		let pos = get_random_position(GAME_FIELD_SIZE[0] as f64, GAME_FIELD_SIZE[1] as f64);

		Enemy::new(pos.0, pos.1) // FIXME Does not check for collision 
	}
}

use crate::{
    coins::Coin,
    enemies::Enemy,
    game_data::{get_random_position, GAME_FIELD_SIZE},
    space::{point_inside_rect, Vec2},
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

    fn spawn_coin(&self) -> Coin {
        loop {
            let pos = get_random_position(GAME_FIELD_SIZE.into());

            if !self.check_collision(pos) {
                return Coin::new(pos.x, pos.y);
            }
        }
    }

    fn spawn_enemy(&self) -> Enemy {
        loop {
            let pos = get_random_position(GAME_FIELD_SIZE.into());

            if !self.check_collision(pos) {
                return Enemy::new(pos.x, pos.y);
            }
        }
    }

    fn generate_coins(&mut self, dt: f32) {
        self.coins_gen_counter += dt;

        if self.coins_gen_counter >= self.settings.coins_gen_rate
            && self.coins.len() < self.settings.max_coins.into()
        {
            self.coins_gen_counter = 0.0;

            let coin = self.spawn_coin();
            self.coins.push(coin)
        }
    }

    fn generate_enemies(&mut self, dt: f32) {
        self.enemies_gen_counter += dt;

        if self.enemies_gen_counter >= self.settings.enemies_gen_rate
            && self.enemies.len() < self.settings.max_enemies.into()
        {
            self.enemies_gen_counter = 0.0;

            let enemy = self.spawn_enemy();
            self.enemies.push(enemy)
        }
    }

    fn init_coins(&mut self) {
        loop {
            let coin = self.spawn_coin();
            self.coins.push(coin);

            if self.coins.len() >= self.settings.max_coins.into() {
                break;
            }
        }
    }

	fn init_enemies(&mut self) {
        loop {
            let enemy = self.spawn_enemy();
            self.enemies.push(enemy);

            if self.enemies.len() >= self.settings.max_enemies.into() {
                break;
            }
        }
	}

	pub fn init(&mut self) {
		self.init_coins();
		self.init_enemies();
	}

	pub fn do_tick(&mut self, dt: f32) {
		self.generate_coins(dt);
		// self.generate_enemies(dt);
	}

    fn check_coin_collision(&self, pos: Vec2) -> bool {
        self.coins
            .iter()
            .any(|&coin| point_inside_rect(coin.pos, coin.size, pos))
    }

    fn check_enemy_collision(&self, pos: Vec2) -> bool {
        self.enemies
            .iter()
            .any(|&enemy| point_inside_rect(enemy.pos, enemy.size, pos))
    }

    fn check_collision(&self, pos: Vec2) -> bool {
        self.check_coin_collision(pos) || self.check_enemy_collision(pos)
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

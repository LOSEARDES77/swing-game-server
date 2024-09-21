use rand::{random, thread_rng, Rng};

use crate::{game_data::GAME_FIELD_SIZE, space::Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub pos: Vec2,
    pub size: Vec2,

	pub speed: f64,

    counter: i16,
    change: i16,

    movement_axis: (bool, bool),
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(1.0, 1.0),
            counter: 0,
            change: 0,
            movement_axis: (false, false),
            speed: 3.0,
        }
    }

    pub fn do_tick(&mut self, dt: f32) {
        //  TEMP Ported from java code, should change this to be based in dt
        let mut rng = thread_rng();

        self.counter = self.counter.saturating_add(dt.round() as i16); // For when dt > i16::MAX

        if self.counter > self.change {
            self.movement_axis = (rng.gen(), rng.gen());
            self.counter = 0;
            self.change = rng.gen_range(100..600);
        }

        let result: bool;

        if self.movement_axis.0 {
            if self.movement_axis.1 {
                if self.pos.x + self.size.x > GAME_FIELD_SIZE[0].into() {
                    self.movement_axis.1 = false;
                }
            // TODO do movement
            } else {
                if self.pos.x < 0.0 {
                    self.movement_axis.1 = true;
                }
                // TODO do movement
            }
        } else {
            if self.movement_axis.1 {
                if self.pos.x + self.size.x > GAME_FIELD_SIZE[0].into() {
                    self.movement_axis.1 = false;
                }
            // TODO do movement
            } else {
                if self.pos.x < 0.0 {
                    self.movement_axis.1 = true;
                }
                // TODO do movement
            }
        }
    }

    fn move_enemy(&mut self, dir: Direction) -> bool {
        let original_pos = self.pos.clone();
        let mut pos = original_pos.clone();

        match dir {
            Direction::UP => pos.y -= self.speed,
            Direction::DOWN => pos.y += self.speed,
            Direction::LEFT => pos.x -= self.speed,
            Direction::RIGHT => pos.x += self.speed,
        };

		// TODO Do collision detection

		self.pos = pos;

		return false;
    }
}

use ggez::nalgebra::Point2;
use ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode, Rect};
use ggez::{Context, GameResult};
use rand::prelude::*;
use crate::Paddle;

enum RocketState {
	Waiting,
	Flying
}

pub enum RocketDirection {
	Left,
	Right
}

pub struct Rocket {
	location: Point2<f32>,
	waiting_location: Point2<f32>,
	color: Color,
	width: f32,
	height: f32,
	state: RocketState,
	velocity: Point2<f32>,
	acceleration: Point2<f32>
}

impl Rocket {
	pub fn new(location: Point2<f32>, direction: RocketDirection) -> Rocket {
		let mut rng = rand::thread_rng();

		Rocket {
			location,
			waiting_location: location,
			color: Color::from_rgb(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)),
			width: 5.0,
			height: 12.0,
			state: RocketState::Waiting,
			velocity: Point2::new(0.0, 0.0),
			acceleration: match direction {
				RocketDirection::Left => Point2::new(-10.0, 0.0),
				RocketDirection::Right => Point2::new(10.0, 0.0)
			}
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		let rocket = match self.state {
			RocketState::Waiting => Rect::new(self.waiting_location.x, self.waiting_location.y, self.width, self.height),
			RocketState::Flying => Rect::new(self.location.x, self.location.y, self.height, self.width)
		};

		MeshBuilder::new()
			.rectangle(DrawMode::fill(), rocket, self.color)
			.build(context)
	}

	pub fn can_fire(&self) -> bool {
		match self.state {
			RocketState::Waiting => true,
			RocketState::Flying => false
		}
	}

	pub fn fire(&mut self, start_location: Point2<f32>) {
		self.state = RocketState::Flying;
		self.location = start_location;
		self.velocity = Point2::new(0.0, 0.0);
	}

	pub fn update(&mut self, delta_time: f32, arena_width: f32, target: &mut Paddle, my_paddle: &mut Paddle) {
		if let RocketState::Flying = self.state {
			self.velocity.x = self.velocity.x + (self.acceleration.x * delta_time);
			self.location.x = self.location.x + self.velocity.x;

			if self.location.x < 0.0 || self.location.x > arena_width {
				self.state = RocketState::Waiting;
			}

			if self.hits_target(target) {
				target.shrink();
				my_paddle.grow();
				self.state = RocketState::Waiting;
			}
		}
	}

	fn hits_target(&self, target: &Paddle) -> bool {
		if self.location.y > target.location.y && self.location.y + self.width < target.location.y + target.height {
			if self.location.x + self.height > target.location.x && self.location.x < target.location.x + target.width {
				true
			} else {
				false
			}
		} else {
			false
		}
	}
}
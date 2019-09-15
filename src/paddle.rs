use ggez::nalgebra::Point2;
use ggez::graphics::{Mesh, MeshBuilder, DrawMode, Rect, Color};
use ggez::{GameResult, Context, input, event};
use crate::Ball;

pub struct Paddle {
	pub location: Point2<f32>,
	speed: f32,
	pub width: f32,
	pub height: f32,
	color: Color,
	is_player: bool
}

enum Direction {
	Up,
	Down,
	Still
}

impl Paddle {
	pub fn new((arena_width, arena_height): (f32, f32), is_player: bool) -> Paddle {
		let height = 75.0;
		let width = 5.0;
		let location = match is_player {
			true => Point2::new(5.0, arena_height / 2.0 - height / 2.0),
			false => Point2::new(arena_width - 5.0 - width, arena_height / 2.0 - height / 2.0)
		};

		Paddle {
			location,
			speed: 400.0,
			width,
			height,
			color: Color::from_rgb(255, 255, 255),
			is_player
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		let rect = Rect::new(self.location.x, self.location.y, self.width, self.height);

		MeshBuilder::new()
			.rectangle(DrawMode::fill(), rect, self.color)
			.build(context)
	}

	pub fn update(&mut self, context: &mut Context, delta_time: f32, arena_size: (f32, f32), ball: &mut Ball) {
		match self.move_direction(context, ball) {
			Direction::Up => self.move_paddle_up(delta_time),
			Direction::Down => self.move_paddle_down(delta_time),
			Direction::Still => ()
		};

		self.limit_paddle_to_arena(arena_size);

		match self.ball_moving_towards_paddle(ball) && self.colliding_with_ball(ball) {
			true => ball.collide_with_paddle(),
			false => ()
		};
	}

	fn move_direction(&mut self, context: &mut Context, ball: &Ball) -> Direction {
		match self.is_player {
			true => self.get_keyboard_direction(context),
			false => self.get_ai_direction(ball)
		}
	}

	fn move_paddle_up(&mut self, delta_time: f32) {
		self.location.y = self.location.y - (self.speed * delta_time);
	}

	fn move_paddle_down(&mut self, delta_time: f32) {
		self.location.y = self.location.y + (self.speed * delta_time);
	}

	fn limit_paddle_to_arena(&mut self, (_, arena_height): (f32, f32)) {
		if self.location.y < 0.0 {
			self.location.y = 0.0;
		} else if self.location.y + self.height > arena_height {
			self.location.y = arena_height - self.height;
		}
	}

	fn get_keyboard_direction(&self, context: &mut Context) -> Direction {
		let pressed_keys = input::keyboard::pressed_keys(context);

        if pressed_keys.contains(&event::KeyCode::Up) {
            Direction::Up
        } else if pressed_keys.contains(&event::KeyCode::Down) {
            Direction::Down
		} else {
			Direction::Still
		}
	}

	fn get_ai_direction(&self, ball: &Ball) -> Direction {
		if ball.location.y < self.location.y {
			Direction::Up
		} else if ball.location.y > self.location.y + self.height {
			Direction::Down
		} else {
			Direction::Still
		}
	}

	fn ball_moving_towards_paddle(&self, ball: &Ball) -> bool {
		if (self.is_player && ball.velocity.x < 0.0) || (!self.is_player && ball.velocity.x > 0.0) {
			true
		} else {
			false
		}
	}

	fn colliding_with_ball(&self, ball: &Ball) -> bool {
		if ball.location.y > self.location.y && ball.location.y < self.location.y + self.height {
			match self.is_player {
				true => ball.location.x - ball.radius < self.location.x + self.width && ball.location.x + ball.radius > self.location.x,
				false => ball.location.x + ball.radius > self.location.x && ball.location.x - ball.radius < self.location.x + self.width
			}
		} else {
			false
		}
	}

	pub fn grow(&mut self) {
		self.height = self.height + 5.0;
	}

	pub fn shrink(&mut self) {
		self.height = self.height - 5.0;
	}
}
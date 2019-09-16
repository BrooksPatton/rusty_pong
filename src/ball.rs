use ggez::nalgebra::{Point2, Vector2};
use ggez::graphics::{Color, WHITE, MeshBuilder, Mesh, DrawMode};
use ggez::{Context, GameResult};

pub struct Ball {
	pub location: Vector2<f32>,
    pub radius: f32,
    color: Color,
    pub velocity: Vector2<f32>,
	speed: f32,
	start_speed: f32
}

impl Ball {
	pub fn new((arena_width, arena_height): (f32, f32)) -> Ball {
		let velocity = Vector2::new(250.0, 100.0);
		let velocity = velocity.normalize();
		let start_speed = 200.0;
		let velocity = velocity * start_speed;

		Ball {
			location: Vector2::new(arena_width / 2.0, arena_height / 2.0),
			radius: 5.0,
			color: WHITE,
			velocity,
			speed: start_speed,
			start_speed
		}
	}

	pub fn update(&mut self, delta_time: f32, arena_size: (f32, f32)) {
		self.move_ball(delta_time);
		self.bounce_ball(arena_size);
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::from(self.location), self.radius, 0.01, self.color)
			.build(context)
	}

	fn move_ball(&mut self, delta_time: f32) {
		self.location.x = self.location.x + (self.velocity.x * delta_time);
		self.location.y = self.location.y + (self.velocity.y * delta_time);
	}

	fn bounce_ball(&mut self, (_, arena_height): (f32, f32)) {
		if self.location.y + self.radius > arena_height {
			self.location.y = arena_height - self.radius;
			self.reverse_y_velocity();
		} else if self.location.y - self.radius < 0.0 {
			self.location.y = self.radius;
			self.reverse_y_velocity();
		}
	}

	fn reverse_x_velocity(&mut self) {
		self.velocity.x = self.velocity.x * -1.0;
	}

	fn reverse_y_velocity(&mut self) {
		self.velocity.y = self.velocity.y * -1.0;
	}

	pub fn reset(&mut self, (arena_width, arena_height): (f32, f32)) {
		self.location.x = arena_width / 2.0;
		self.location.y = arena_height / 2.0;
		self.velocity = Vector2::new(350.0, 150.0).normalize() * self.start_speed;
		self.speed = self.start_speed;
	}

	pub fn collide_with_paddle(&mut self, how_far_off_center: f32) {
		self.reverse_x_velocity();
		self.velocity.y = how_far_off_center / 25.0;
		self.velocity.x = if self.velocity.x > 0.0 {
			1.0
		} else {
			-1.0
		};

		self.increase_speed();
	}

	fn increase_speed(&mut self) {
		self.speed = self.speed + 50.0;
		self.velocity = self.velocity.normalize() * self.speed;
	}
}

use ggez::nalgebra::{Point2};
use ggez::graphics::{Color, WHITE, MeshBuilder, Mesh, DrawMode};
use ggez::{Context, GameResult};

pub struct Ball {
	pub location: Point2<f32>,
    pub radius: f32,
    color: Color,
    pub velocity: Point2<f32>
}

impl Ball {
	pub fn new((arena_width, arena_height): (f32, f32)) -> Ball {
		Ball {
			location: Point2::new(arena_width / 2.0, arena_height / 2.0),
			radius: 5.0,
			color: WHITE,
			velocity: Point2::new(350.0, 150.0)
		}
	}

	pub fn update(&mut self, delta_time: f32, arena_size: (f32, f32)) {
		self.move_ball(delta_time);
		self.bounce_ball(arena_size);
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), self.location, self.radius, 0.01, self.color)
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

	pub fn reverse_x_velocity(&mut self) {
		self.velocity.x = self.velocity.x * -1.0;
	}

	fn reverse_y_velocity(&mut self) {
		self.velocity.y = self.velocity.y * -1.0;
	}

	pub fn reset(&mut self, (arena_width, arena_height): (f32, f32)) {
		self.location.x = arena_width / 2.0;
		self.location.y = arena_height / 2.0;
		self.velocity = Point2::new(350.0, 150.0)
	}
}

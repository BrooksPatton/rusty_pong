use ggez::nalgebra::Point2;

pub struct Paddle {
	location: Point2<f32>,
	speed: f32,
	width: f32,
	height: f32,
	is_player: bool
}

impl Paddle {
	pub fn new(arena_width: f32, arena_height: f32, is_player: bool) -> Paddle {
		let height = 75.0;
		let width = 5.0;
		let location = match is_player {
			true => Point2::new(5.0, arena_height / 2.0 - height / 2.0),
			false => Point2::new(arena_width - 5.0 - width, arena_height / 2.0 - height / 2.0)
		};

		Paddle {
			location,
			speed: 50.0,
			width,
			height,
			is_player	
		}
	}

	pub fn update()
}
use ggez::{ContextBuilder, event, Context, GameResult, graphics};
use ggez::event::{EventHandler};
use ggez::nalgebra::{Point2};

struct Pong {
    ball_location: Point2<f32>,
    ball_relative_location: Point2<f32>,
    ball_radius: f32,
    ball_color: graphics::Color
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            ball_location: Point2::new(50.0, 65.0),
            ball_relative_location: Point2::new(0.0, 0.0),
            ball_radius: 100.0,
            ball_color: graphics::WHITE
        }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let ball = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(), 
                self.ball_relative_location, 
                self.ball_radius, 
                0.01, 
                self.ball_color)
            .build(context)?;

        graphics::draw(context, &ball, (self.ball_location,))?;

        graphics::present(context)
    }
}

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("rusty_pong", "Brooks Patton")
        .build()
        .expect("game context not able to be created :(");

    let mut pong = Pong::new();

    match event::run(&mut context, &mut event_loop, &mut pong) {
        Ok(_) => println!("Game finished, hope you had fun!"),
        Err(error) => println!("Error playing the game: {}", error)
    };
}

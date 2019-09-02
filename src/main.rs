use ggez::{ContextBuilder, event, Context, GameResult, graphics, timer};
use ggez::event::{EventHandler};
use ggez::nalgebra::{Point2};

struct Pong {
    ball_location: Point2<f32>,
    ball_relative_location: Point2<f32>,
    ball_radius: f32,
    ball_color: graphics::Color,
    ball_velocity: Point2<f32>
}

impl Pong {
    pub fn new(context: &Context) -> Pong {
        let (window_width, window_height) = graphics::drawable_size(context);

        let ball_velocity = Point2::new(300.0, -150.0);

        Pong {
            ball_location: Point2::new(window_width / 2.0, window_height / 2.0),
            ball_relative_location: Point2::new(0.0, 0.0),
            ball_radius: 10.0,
            ball_color: graphics::WHITE,
            ball_velocity
        }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let duration = timer::delta(context);
        let delta_time = duration.as_nanos() as f32 / 1e9;

        self.ball_location = Point2::new(
            self.ball_location.x + (self.ball_velocity.x * delta_time),
            self.ball_location.y + (self.ball_velocity.y * delta_time)
        );

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

    let mut pong = Pong::new(&context);

    match event::run(&mut context, &mut event_loop, &mut pong) {
        Ok(_) => println!("Game finished, hope you had fun!"),
        Err(error) => println!("Error playing the game: {}", error)
    };
}

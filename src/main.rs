use ggez::{ContextBuilder, event, Context, GameResult, graphics, timer, input};
use ggez::event::{EventHandler};
use ggez::nalgebra::{Point2};

struct Pong {
    ball_location: Point2<f32>,
    ball_radius: f32,
    ball_color: graphics::Color,
    ball_velocity: Point2<f32>,
    window_width: f32,
    window_height: f32,
    player_paddle_location: graphics::Rect,
    paddle_width: f32,
    paddle_height: f32,
    player_paddle_color: graphics::Color,
    paddle_speed: f32
}

impl Pong {
    pub fn new(context: &Context) -> Pong {
        let (window_width, window_height) = graphics::drawable_size(context);
        let ball_velocity = Point2::new(300.0, -150.0);
        let paddle_height = 150.0;
        let player_paddle_location = graphics::Rect::new(10.0, window_height / 2.0 - paddle_height / 2.0, 10.0, paddle_height);
        let paddle_width = 10.0;
        let player_paddle_color = graphics::WHITE;
        let paddle_speed = 500.0;

        Pong {
            ball_location: Point2::new(window_width / 2.0, window_height / 2.0),
            ball_radius: 10.0,
            ball_color: graphics::WHITE,
            ball_velocity,
            window_width,
            window_height,
            player_paddle_location,
            paddle_height,
            paddle_width,
            player_paddle_color,
            paddle_speed
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

        if self.ball_location.x > self.window_width - self.ball_radius {
            self.ball_location.x = self.window_width - self.ball_radius;
            self.ball_velocity.x = self.ball_velocity.x * -1.0;
        } else if self.ball_location.x < self.ball_radius {
            self.ball_location.x = self.ball_radius;
            self.ball_velocity.x = self.ball_velocity.x * -1.0;
        } else if self.ball_location.y < self.ball_radius {
            self.ball_location.y = self.ball_radius;
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        } else if self.ball_location.y > self.window_height - self.ball_radius {
            self.ball_location.y = self.window_height - self.ball_radius;
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }

        let pressed_keys = input::keyboard::pressed_keys(context);

        if pressed_keys.contains(&event::KeyCode::Up) {
            self.player_paddle_location.y = self.player_paddle_location.y - (self.paddle_speed * delta_time);
        } else if pressed_keys.contains(&event::KeyCode::Down) {
            self.player_paddle_location.y = self.player_paddle_location.y + (self.paddle_speed * delta_time);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let mesh = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(), 
                self.ball_location, 
                self.ball_radius, 
                0.01, 
                self.ball_color)
            .rectangle(graphics::DrawMode::fill(), 
                self.player_paddle_location, 
                self.player_paddle_color
            )
            .build(context)?;

        graphics::draw(context, &mesh, (Point2::new(0.0, 0.0),))?;

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

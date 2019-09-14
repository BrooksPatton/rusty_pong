mod paddle;
mod ball;

use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult, timer};
use ggez::event::EventHandler;
use paddle::Paddle;
use ball::Ball;

pub struct Pong {
    player_paddle: Paddle,
    ball: Ball,
    arena_size: (f32, f32),
    ai_paddle: Paddle
}

impl Pong {
    pub fn new(context: &Context) -> Pong {
        let arena_size = graphics::drawable_size(context);

        Pong {
            ball: Ball::new(arena_size),
            player_paddle: Paddle::new(arena_size, true),
            arena_size,
            ai_paddle: Paddle::new(arena_size, false)
        }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let duration = timer::delta(context);
        let delta_time = duration.as_nanos() as f32 / 1e9;

        self.ball.update(delta_time, self.arena_size);
        self.player_paddle.update(context, delta_time, self.arena_size, &mut self.ball);
        self.ai_paddle.update(context, delta_time, self.arena_size, &mut self.ball);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let player_paddle = self.player_paddle.draw(context)?;
        let ball = self.ball.draw(context)?;
        let ai_paddle = self.ai_paddle.draw(context)?;

        graphics::draw(context, &ball, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &player_paddle, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ai_paddle, (Point2::new(0.0, 0.0),))?;

        graphics::present(context)
    }
}
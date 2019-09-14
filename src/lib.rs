mod paddle;
mod ball;

use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult, timer};
use ggez::event::EventHandler;
use ggez::graphics::{Rect, Color, MeshBuilder, Mesh, DrawMode, Text, TextFragment, WHITE, Scale};
use paddle::Paddle;
use ball::Ball;

enum PlayingState {
    Playing,
    Scored
}

pub struct Pong {
    player_paddle: Paddle,
    ball: Ball,
    arena_size: (f32, f32),
    ai_paddle: Paddle,
    player_score: u8,
    ai_score: u8,
    state: PlayingState
}

impl Pong {
    pub fn new(context: &Context) -> Pong {
        let arena_size = graphics::drawable_size(context);

        Pong {
            ball: Ball::new(arena_size),
            player_paddle: Paddle::new(arena_size, true),
            arena_size,
            ai_paddle: Paddle::new(arena_size, false),
            player_score: 0,
            ai_score: 0,
            state: PlayingState::Playing
        }
    }

    fn create_center_line(&self, context: &mut Context) -> GameResult<Mesh> {
        let width = 15.0;
        let center_line = Rect::new(self.arena_size.0 / 2.0 - width / 2.0, 0.0, width, self.arena_size.1);

        MeshBuilder::new()
            .rectangle(DrawMode::fill(), center_line, Color::from_rgba(255, 255, 255, 10))
            .build(context)
    }

    fn create_score(&self, score: u8) -> TextFragment {
        TextFragment::new(score.to_string())
            .color(WHITE)
            .scale(Scale::uniform(50.0))
    }
}

impl EventHandler for Pong {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let duration = timer::delta(context);
        let delta_time = duration.as_nanos() as f32 / 1e9;
        let pressed_keys = ggez::input::keyboard::pressed_keys(context);
        
        match self.state {
            PlayingState::Playing => {

                self.ball.update(delta_time, self.arena_size);

                let (arena_width, _) = self.arena_size;

                if self.ball.location.x < 0.0 {
                    self.ai_score = self.ai_score + 1;
                    self.state = PlayingState::Scored;
                    self.ball.reset(self.arena_size);
                } else if self.ball.location.x > arena_width {
                    self.player_score = self.player_score + 1;
                    self.state = PlayingState::Scored;
                    self.ball.reset(self.arena_size);
                }
            },
            PlayingState::Scored => {
                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.state = PlayingState::Playing;
                }
            }
        };

        self.player_paddle.update(context, delta_time, self.arena_size, &mut self.ball);
        self.ai_paddle.update(context, delta_time, self.arena_size, &mut self.ball);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let center_line = self.create_center_line(context)?;
        let player_paddle = self.player_paddle.draw(context)?;
        let ball = self.ball.draw(context)?;
        let ai_paddle = self.ai_paddle.draw(context)?;
        let player_score = Text::new(self.create_score(self.player_score));
        let ai_score = Text::new(self.create_score(self.ai_score));

        graphics::draw(context, &center_line, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &player_score, (Point2::new(self.arena_size.0 / 2.0 - 50.0, 10.0),))?;
        graphics::draw(context, &ai_score, (Point2::new(self.arena_size.0 / 2.0 + 25.0, 10.0),))?;
        graphics::draw(context, &ball, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &player_paddle, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ai_paddle, (Point2::new(0.0, 0.0),))?;

        match self.state {
            PlayingState::Scored => {
                let start_again_text = TextFragment::new("Press space to play")
                    .color(WHITE)
                    .scale(Scale::uniform(50.0));
                let start_again_text = Text::new(start_again_text);
                let (text_width, _text_height) = start_again_text.dimensions(context);

                graphics::draw(context, &start_again_text, (Point2::new(self.arena_size.0 / 2.0 - (text_width / 2) as f32, 100.0),))?;
            },
            _ => ()
        }

        graphics::present(context)
    }

}

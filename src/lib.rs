mod paddle;
mod ball;
mod rocket;

use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult, timer};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Rect, Color, MeshBuilder, Mesh, DrawMode, Text, TextFragment, WHITE, Scale};
use paddle::Paddle;
use ball::Ball;
use rocket::{Rocket, RocketDirection};

enum PlayingState {
    NotStarted,
    Playing,
    Scored,
    GameOver
}

pub struct Pong {
    player_paddle: Paddle,
    ball: Ball,
    arena_size: (f32, f32),
    ai_paddle: Paddle,
    player_score: u8,
    ai_score: u8,
    state: PlayingState,
    max_score: u8,
    player_rocket: Rocket,
    ai_rocket: Rocket
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
            state: PlayingState::NotStarted,
            max_score: 10,
            player_rocket: Rocket::new(Point2::new(arena_size.0 / 2.0 - 50.0, arena_size.1 - 15.0), RocketDirection::Right),
            ai_rocket: Rocket::new(Point2::new(arena_size.0 / 2.0 + 145.0, arena_size.1 - 15.0), RocketDirection::Left)
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

    fn scored(&mut self) {
        if self.player_score >= self.max_score ||self.ai_score >= self.max_score {
            self.state = PlayingState::GameOver;
        } else {
            self.state = PlayingState::Scored;
            self.ball.reset(self.arena_size);
        }
    }

    fn create_rocket_space(&mut self) -> Text {
        let rocket_title = TextFragment::new("rockets:")
            .color(Color::from_rgba(255, 255, 255, 50))
            .scale(Scale::uniform(25.0));

        Text::new(rocket_title)
    }
}

impl EventHandler for Pong {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let duration = timer::delta(context);
        let delta_time = duration.as_nanos() as f32 / 1e9;
        let pressed_keys = ggez::input::keyboard::pressed_keys(context);
        
        match self.state {
            PlayingState::NotStarted => {
                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.state = PlayingState::Playing;
                }
            },
            PlayingState::Playing => {

                self.ball.update(delta_time, self.arena_size);

                let (arena_width, _) = self.arena_size;

                if self.ball.location.x < 0.0 {
                    self.ai_score = self.ai_score + 1;
                    self.scored();
                } else if self.ball.location.x > arena_width {
                    self.player_score = self.player_score + 1;
                    self.scored();
                }

                if self.ai_rocket.can_fire() {
                    self.ai_rocket.fire(Point2::new(self.arena_size.0, self.ai_paddle.location.y + self.ai_paddle.height / 2.0));
                }

                if self.player_rocket.can_fire() && pressed_keys.contains(&KeyCode::LShift) {
                    self.player_rocket.fire(Point2::new(0.0, self.player_paddle.location.y + self.player_paddle.height / 2.0));
                }
            },
            PlayingState::Scored => {
                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.state = PlayingState::Playing;
                }
            },
            PlayingState::GameOver => {
                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.ball.reset(self.arena_size);
                    self.ai_score = 0;
                    self.player_score = 0;
                    self.state = PlayingState::Playing;
                }
            }
        };

        self.player_paddle.update(context, delta_time, self.arena_size, &mut self.ball);
        self.ai_paddle.update(context, delta_time, self.arena_size, &mut self.ball);
        self.ai_rocket.update(delta_time, self.arena_size.0, &mut self.player_paddle, &mut self.ai_paddle);
        self.player_rocket.update(delta_time, self.arena_size.0, &mut self.ai_paddle, &mut self.player_paddle);

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
        let rocket_title = self.create_rocket_space();
        let player_rocket = self.player_rocket.draw(context)?;
        let ai_rocket = self.ai_rocket.draw(context)?;

        graphics::draw(context, &center_line, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &player_score, (Point2::new(self.arena_size.0 / 2.0 - 50.0, 10.0),))?;
        graphics::draw(context, &ai_score, (Point2::new(self.arena_size.0 / 2.0 + 25.0, 10.0),))?;
        graphics::draw(context, &player_rocket, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ai_rocket, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ball, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &player_paddle, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &ai_paddle, (Point2::new(0.0, 0.0),))?;
        graphics::draw(context, &rocket_title, (Point2::new(self.arena_size.0 / 2.0 - 150.0, self.arena_size.1 - 25.0),))?;
        graphics::draw(context, &rocket_title, (Point2::new(self.arena_size.0 / 2.0 + 50.0, self.arena_size.1 - 25.0),))?;

        match self.state {
            PlayingState::NotStarted => {
                let start_game = TextFragment::new("Press space to begin playing")
                    .color(WHITE)
                    .scale(Scale::uniform(50.0));
                let instructions = TextFragment::new("Press Left Shift to fire a rocket")
                    .color(WHITE)
                    .scale(Scale::uniform(40.0));
                let start_game = Text::new(start_game);
                let instructions = Text::new(instructions);
                let (start_game_width, _) = start_game.dimensions(context);
                let (instruction_width, _) = instructions.dimensions(context);

                graphics::draw(context, &start_game, (Point2::new(self.arena_size.0 / 2.0 - (start_game_width / 2) as f32, 100.0),))?;
                graphics::draw(context, &instructions, (Point2::new(self.arena_size.0 / 2.0 - (instruction_width / 2) as f32, 150.0),))?;
            },
            PlayingState::Scored => {
                let start_again_text = TextFragment::new("Press space to play")
                    .color(WHITE)
                    .scale(Scale::uniform(50.0));
                let start_again_text = Text::new(start_again_text);
                let (text_width, _text_height) = start_again_text.dimensions(context);

                graphics::draw(context, &start_again_text, (Point2::new(self.arena_size.0 / 2.0 - (text_width / 2) as f32, 100.0),))?;
            },
            PlayingState::GameOver => {
                let text = match self.player_score >= self.max_score {
                    true => "You Win! Press space to play again",
                    false => "You lose! Press space to try again"
                };
                let start_again_text = TextFragment::new(text)
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

use ggez::{ContextBuilder, event, Context, GameResult, graphics};
use ggez::event::{EventHandler};

struct Pong {

}

impl Pong {
    pub fn new() -> Pong {
        Pong {}
    }
}

impl EventHandler for Pong {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

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

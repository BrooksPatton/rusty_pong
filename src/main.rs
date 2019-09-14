use ggez::{ContextBuilder, event};
use rusty_pong::*;

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

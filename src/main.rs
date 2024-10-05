use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct GameState {}

impl GameState {
    fn new() -> GameResult<GameState> {
        let state = GameState {};
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Alone in Space", "Space Nerd");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}

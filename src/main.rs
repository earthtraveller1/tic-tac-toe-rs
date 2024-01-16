use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
    ContextBuilder, GameError, GameResult,
};

struct Game {}

impl Game {
    fn new() -> GameResult<Game> {
        Ok(Game {})
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }
}

pub fn main() -> GameResult {
    let (context, event_loop) = ContextBuilder::new("tic-tac-toe", "earthtraveller1")
        .window_setup(WindowSetup {
            title: "Tic Tac Toe".to_string(),
            ..Default::default()
        })
        .build()?;

    let game = Game::new()?;
    event::run(context, event_loop, game)
}

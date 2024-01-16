use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, Mesh, Rect},
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

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), GameError> {
        let factor: f32 = 0.90;
        let mut canvas = Canvas::from_frame(context, Color::new(factor, factor, factor, factor));

        let test = Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            Rect {
                x: 0.0,
                y: 0.0,
                w: 100.0,
                h: 100.0,
            },
            Color::BLUE,
        )?;

        canvas.draw(&test, Vec2::new(100.0, 100.0));

        canvas.finish(context)?;
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

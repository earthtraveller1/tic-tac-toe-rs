use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image},
    ContextBuilder, GameError, GameResult,
};

struct Game {
    can_pooper_image: Image,
    angry_pooper_image: Image,
}

impl Game {
    fn new(context: &ggez::Context) -> GameResult<Game> {
        Ok(Game {
            can_pooper_image: Image::from_path(context, "/canpooper_right.png")?,
            angry_pooper_image: Image::from_path(context, "/canpooper_right_angry.png")?,
        })
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), GameError> {
        let factor: f32 = 0.90;
        let mut canvas = Canvas::from_frame(context, Color::new(factor, factor, factor, factor));

        let scale = Vec2::new(0.25, 0.25);

        canvas.draw(
            &self.can_pooper_image,
            DrawParam::new()
                .dest(Vec2::new(100.0, 100.0))
                .scale(scale),
        );

        canvas.draw(
            &self.angry_pooper_image,
            DrawParam::new()
                .dest(Vec2::new(400.0, 100.0))
                .scale(scale),
        );

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
        .add_resource_path("./assets")
        .build()?;

    let game = Game::new(&context)?;
    event::run(context, event_loop, game)
}

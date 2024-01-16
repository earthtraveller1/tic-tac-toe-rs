use ggez::{
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect},
    ContextBuilder, GameError, GameResult,
};

struct Game {
    can_pooper_image: Image,
    angry_pooper_image: Image,
    grid_line: Mesh,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

impl Game {
    fn new(context: &ggez::Context) -> GameResult<Game> {
        Ok(Game {
            can_pooper_image: Image::from_path(context, "/canpooper_right.png")?,
            angry_pooper_image: Image::from_path(context, "/canpooper_right_angry.png")?,
            grid_line: Mesh::new_rounded_rectangle(
                &context.gfx,
                DrawMode::fill(),
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 400.0,
                    h: 10.0,
                },
                5.0,
                Color::new(0.1, 0.1, 0.25, 1.0),
            )?,
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

        let scale_factor = 0.20;
        let scale = Vec2::new(scale_factor, scale_factor);

        let cell_size = 400.0 * 1.0 / 3.0;

        {
            // Draw the grid

            canvas.draw(
                &self.grid_line,
                DrawParam::new().dest(Vec2::new(200.0, 120.0 + cell_size)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new().dest(Vec2::new(200.0, 120.0 + 2.0 * cell_size)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new()
                    .rotation(90.0f32.to_radians())
                    .dest(Vec2::new(200.0 + cell_size, 120.0)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new()
                    .rotation(90.0f32.to_radians())
                    .dest(Vec2::new(200.0 + 2.0 * cell_size, 120.0)),
            );
        }

        {
            // Draw the characters
            let padding = 10.0;

            let x_locations = [
                200.0 + padding,
                200.0 + cell_size + padding,
                200.0 + 2.0 * cell_size + padding,
            ];

            let y_locations = [
                120.0 + padding,
                120.0 + cell_size + padding,
                120.0 + 2.0 * cell_size + padding,
            ];

            for x in x_locations {
                for y in y_locations {
                    canvas.draw(
                        &self.can_pooper_image,
                        DrawParam::new().dest(Vec2::new(x, y)).scale(scale),
                    );
                }
            }

            canvas.draw(
                &self.angry_pooper_image,
                DrawParam::new().dest(Vec2::new(400.0, 100.0)).scale(scale),
            );
        }

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
        .window_mode(WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_resource_path("./assets")
        .build()?;

    let game = Game::new(&context)?;
    event::run(context, event_loop, game)
}

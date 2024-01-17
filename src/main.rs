use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect},
    ContextBuilder, GameError, GameResult,
};

#[derive(Default)]
enum CellState {
    #[default]
    CanPooper,
    AngryPooper,
}

struct Game {
    can_pooper_image: Image,
    angry_pooper_image: Image,
    grid_line: Mesh,
    cells: [[CellState; 3]; 3],
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const GAME_WIDTH: f32 = 400.0;
const GAME_HEIGHT: f32 = 400.0;
const GAME_MARGIN_X: f32 = (WINDOW_WIDTH - GAME_WIDTH) / 2.0;
const GAME_MARGIN_Y: f32 = (WINDOW_HEIGHT - GAME_HEIGHT) / 2.0;
const CELL_SIZE: f32 = GAME_WIDTH * 1.0 / 3.0;

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
            cells: Default::default(),
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

        {
            // Draw the grid

            canvas.draw(
                &self.grid_line,
                DrawParam::new().dest(Vec2::new(GAME_MARGIN_X, GAME_MARGIN_Y + CELL_SIZE)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new().dest(Vec2::new(GAME_MARGIN_X, GAME_MARGIN_Y + 2.0 * CELL_SIZE)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new()
                    .rotation(90.0f32.to_radians())
                    .dest(Vec2::new(GAME_MARGIN_X + CELL_SIZE, 120.0)),
            );

            canvas.draw(
                &self.grid_line,
                DrawParam::new()
                    .rotation(90.0f32.to_radians())
                    .dest(Vec2::new(GAME_MARGIN_X + 2.0 * CELL_SIZE, 120.0)),
            );
        }

        {
            // Draw the characters
            let padding = 10.0;

            self.cells
                .iter()
                .enumerate()
                .for_each(|(cell_y, cell_row)| {
                    let y = GAME_MARGIN_Y + (cell_y as f32) * CELL_SIZE + padding;
                    cell_row.iter().enumerate().for_each(|(cell_x, cell)| {
                        let x = GAME_MARGIN_X + (cell_x as f32) * CELL_SIZE + padding;
                        match cell {
                            CellState::CanPooper => {
                                canvas.draw(
                                    &self.can_pooper_image,
                                    DrawParam::new().dest(Vec2::new(x, y)).scale(scale),
                                );
                            }
                            CellState::AngryPooper => {
                                canvas.draw(
                                    &self.angry_pooper_image,
                                    DrawParam::new().dest(Vec2::new(x, y)).scale(scale),
                                );
                            }
                        }
                    })
                });

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

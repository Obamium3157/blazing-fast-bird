/*
game.rs
В файле содержатся основные параметры игры, а также получает реализацию вся логика
*/

mod physics;
mod game_objects;

use ggez::{graphics, mint, event};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};

use physics::{fall, jump};
use game_objects::Tube;
use game_objects::check_tube;

const PLAYER_SIZE: f32 = 65.0;
const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;


pub struct MainState {
    player_pos: mint::Point2<f32>,
    in_game: bool,
}

impl MainState {
    pub fn new(ctx: &Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        Self {
            player_pos: mint::Point2 { x: screen_w / 2.0, y: screen_h / 2.0 },
            in_game: false,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(&ctx, KeyCode::Space) && !self.in_game {
            self.in_game = true;
        }
        if self.in_game {
            fall(&ctx, &mut self.player_pos);

            if keyboard::is_key_pressed(&ctx, KeyCode::Space) {
                jump(&ctx, &mut self.player_pos)
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::CYAN);

        // let (screen_w, screen_h) = graphics::drawable_size(ctx);

        let player_rect = graphics::Rect::new(
            -PLAYER_SIZE_HALF,
            -PLAYER_SIZE_HALF,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );

        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            graphics::Color::RED,
        )?;

        let draw_param = graphics::DrawParam::default();

        graphics::draw(ctx, &player_mesh, draw_param.dest(self.player_pos))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

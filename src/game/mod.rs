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
use game_objects::{Tube, Player};
use game_objects::check_tube;


pub struct MainState {
    player: Player,
    in_game: bool,
}

impl MainState {
    pub fn new(ctx: &Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        Self {
            player: Player::new(
                mint::Point2::<f32> {
                    x: screen_w / 2.0,
                    y: screen_h / 2.0,
                },
                graphics::Color::RED,
            ),
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
            fall(&ctx, &mut self.player.position);

            if keyboard::is_key_pressed(&ctx, KeyCode::Space) {
                jump(&ctx, &mut self.player.position)
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::CYAN);

        // let (screen_w, screen_h) = graphics::drawable_size(ctx);

        let player_rect = graphics::Rect::new(
            -(self.player.size / 2.0),
            -(self.player.size / 2.0),
            self.player.size,
            self.player.size,
        );

        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            self.player.color,
        )?;

        let draw_param = graphics::DrawParam::default();

        graphics::draw(ctx, &player_mesh, draw_param.dest(self.player.position))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

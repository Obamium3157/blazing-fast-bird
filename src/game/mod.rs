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
use game_objects::{check_tube, gen_random_height};
use game_objects::{TUBES_AMOUNT, TUBES_PADDING};

use self::physics::move_tube;


pub struct MainState {
    player: Player,
    tubes: [Tube ; TUBES_AMOUNT],
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
            tubes: [
                Tube::new(
                    gen_random_height(&ctx),
                    screen_w,
                    [
                        0.0,
                        screen_h
                    ],
                    graphics::Color::GREEN,
                ),
                Tube::new(
                    gen_random_height(&ctx),
                    screen_w + TUBES_PADDING,
                    [
                        0.0,
                        screen_h
                    ],
                    graphics::Color::GREEN,
                ),
                Tube::new(
                    gen_random_height(&ctx),
                    screen_w + TUBES_PADDING * 2.0,
                    [
                        0.0,
                        screen_h
                    ],
                    graphics::Color::GREEN,
                ),
            ],
            in_game: false,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(&ctx, KeyCode::W) && !self.in_game {
            self.in_game = true;
        }
        if self.in_game {
            fall(&ctx, &mut self.player.position);

            if keyboard::is_key_pressed(&ctx, KeyCode::W) {
                jump(&ctx, &mut self.player.position)
            }

            for i in 0..self.tubes.len() {
                move_tube(&ctx, &mut self.tubes[i]);
                check_tube(ctx, &mut self.tubes[i], graphics::drawable_size(ctx).0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);

        // let (screen_w, screen_h) = graphics::drawable_size(ctx);

        let draw_param = graphics::DrawParam::default();

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
        // -(self.tubes[i].get_height()[0] / 2.0),

        for i in 0..self.tubes.len() {
            let tube_rect0 = graphics::Rect::new(
                -(self.tubes[i].get_width() / 2.0),
                -(self.tubes[i].get_height()[0] / 2.0),
                self.tubes[i].get_width(),
                self.tubes[i].get_height()[0]
            );
            let tube_rect1 = graphics::Rect::new(
                -(self.tubes[i].get_width() / 2.0),
                -(self.tubes[i].get_height()[0] / 2.0),
                self.tubes[i].get_width(),
                self.tubes[i].get_height()[1]
            );

            let tube_mesh0 = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(),
                tube_rect0,
                self.tubes[i].get_color()
            )?;
            let tube_mesh1 = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(),
                tube_rect1,
                self.tubes[i].get_color()
            )?;

            graphics::draw(
                ctx,
                &tube_mesh0,
                draw_param.dest(mint::Point2::<f32> {
                    x: self.tubes[i].get_x(),
                    y: self.tubes[i].get_y()[0],
                })
            )?;
            graphics::draw(
                ctx,
                &tube_mesh1,
                draw_param.dest(mint::Point2::<f32> {
                    x: self.tubes[i].get_x(),
                    y: self.tubes[i].get_y()[0],
                })
            )?;
        }

        graphics::draw(ctx, &player_mesh, draw_param.dest(self.player.position))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

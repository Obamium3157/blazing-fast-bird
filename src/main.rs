/*
main.rs
В этом файле программа берет свое начало.
Здесь также задаются базовые параметры, такие, как название игры и размер окна.
*/

mod game;

use ggez::{event, graphics};
use ggez::{ContextBuilder, GameResult};

use game::MainState;

fn main() -> GameResult {
    let c = ggez::conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("Blazing-fast-bird", "obamium3157")
        .default_conf(c)
        .build()
        .unwrap();

    let (screen_w, screen_h) = graphics::drawable_size(&ctx);

    graphics::set_window_title(&ctx, "Flappy bird");
    graphics::set_drawable_size(&mut ctx, screen_w * 2.0, screen_h * 2.0).ok();

    let state = MainState::new(&ctx);
    event::run(ctx, event_loop, state);
}

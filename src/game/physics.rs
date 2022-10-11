/*
physics.rs
В файле содержится вся физика игры.
*/

use ggez::Context;
use ggez::mint;

const JUMP_SPEED: f32 = 2500.0;
const FALL_SPEED: f32 = JUMP_SPEED / 6.0;


pub fn fall(ctx: &Context, pos: &mut mint::Point2<f32>) {
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    pos.y += FALL_SPEED * dt;
}

pub fn jump(ctx: &Context, pos: &mut mint::Point2<f32>) {
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    pos.y -= JUMP_SPEED * dt - FALL_SPEED * dt;
}
/*
physics.rs
В файле содержится вся физика игры.
*/

use ggez::Context;
use ggez::mint::Point2;

use crate::game::game_objects::Tube;

const JUMP_SPEED: f32 = 1000.0;
const FALL_SPEED: f32 = JUMP_SPEED / 4.0;
const TUBE_SPEED: f32 = 150.0;


pub fn fall(ctx: &Context, pos: &mut Point2<f32>) {
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    pos.y += FALL_SPEED * dt;
}

pub fn jump(ctx: &Context, pos: &mut Point2<f32>) {
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    pos.y -= JUMP_SPEED * dt - FALL_SPEED * dt;
}

pub fn move_tube(ctx: &Context, tube: &mut Tube) {
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    tube.set_x(tube.get_x() - TUBE_SPEED * dt);
}

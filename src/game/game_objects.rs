/*
 * game_objects.rs
 * В файле содержатся все игровые объекты (кроме игрока)
 */

use ggez::{mint, graphics, Context};
use rand::Rng;

pub const TUBES_AMOUNT: usize = 3;

const PLAYER_SIZE: f32 = 65.0;
const DOUBLE_PLAYER_SIZE: f32 = PLAYER_SIZE * 2.0;
const PADDING: f32 = PLAYER_SIZE * 4.0;

pub struct Player {
    pub(crate) position: mint::Point2<f32>,
    // rotation: f32,
    pub(crate) color: graphics::Color,
    pub(crate) size: f32,
}

impl Player {
    pub fn new(position: mint::Point2<f32>, color: graphics::Color) -> Self {
        Self {
            position,
            color,
            size: PLAYER_SIZE,
        }
    }
}

pub struct Tube {
    height: [f32 ; 2],           // Высота каждой трубы, .0 -> нижняя, .1 -> верхняя
    width: f32,                  // Длина каждой трубы
    position: mint::Point2<f32>, // Положение трубы в пространстве
    color: graphics::Color,      // Цвет трубы
}

impl Tube {
    pub fn new(
        height: [f32 ; 2],
        position: mint::Point2<f32>,
        color: graphics::Color
    ) -> Self {
        Self {
            height,
            width: DOUBLE_PLAYER_SIZE * 1.5,
            position,
            color,
        }
    }

    pub fn set_position(&mut self, pos: mint::Point2<f32>) {
        self.position = pos;
    }
    pub fn get_position(&self) -> mint::Point2<f32> {
        self.position
    }

    pub fn set_height(&mut self, h: f32, n: usize) {
        self.height[n] = h;
    }
    pub fn get_height(&self) -> [f32 ; 2] {
        self.height
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_color(&self) -> graphics::Color {
        self.color
    }
}

pub fn create_tube(ctx: &Context, tube: &mut Tube) {
    let mut rng = rand::thread_rng();
    let screen_h = graphics::drawable_size(&ctx).1;

    let height1 = screen_h - DOUBLE_PLAYER_SIZE - PADDING;
    let height0 = rng.gen_range(
        DOUBLE_PLAYER_SIZE..height1
    );

    tube.set_height(height0, 0);
    tube.set_height(height1, 1);
}

pub fn gen_random_height(ctx: &Context) -> [f32 ; 2] {
    let mut rng = rand::thread_rng();
    let screen_h = graphics::drawable_size(&ctx).1;

    let height1 = screen_h - DOUBLE_PLAYER_SIZE - PADDING;
    let height0 = rng.gen_range(
        DOUBLE_PLAYER_SIZE..height1
    );

    [height0, height1]
}


pub fn check_tube(ctx: &Context,t: &mut Tube, screen_w: f32) {
    if t.position.x + t.width < 0.0 {
        t.position.x = screen_w + t.width;
        create_tube(ctx, t);
    }
}
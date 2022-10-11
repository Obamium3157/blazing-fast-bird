/*
game_objects.rs
В файле содержатся все игровые объекты (кроме игрока)
*/

use ggez::{mint, graphics};

const PLAYER_SIZE: f32 = 65.0;
const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;

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
    amount: u8,                  // Количество труб на экране (необходимо для оптимизации)
    position: mint::Point2<f32>, // Положение трубы в пространстве
    color: graphics::Color,      // Цвет трубы
}

impl Tube {
    pub fn new(
        height: [f32 ; 2],
        width: f32,
        amount: u8,
        position: mint::Point2<f32>,
        color: graphics::Color
    ) -> Self {
        Self {
            height,
            width,
            amount,
            position,
            color,
        }
    }
}

//TODO: сделать массив с возможными  размерами платформ


pub fn check_tube(t: &mut Tube, screen_w: f32) {
    if t.position.x + t.width < 0.0 {
        t.position.x = screen_w + t.width;
    }
}
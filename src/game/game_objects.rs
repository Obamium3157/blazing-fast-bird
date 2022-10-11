/*
game_objects.rs
В файле содержатся все игровые объекты (кроме игрока)
*/

use ggez::{mint, graphics};

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
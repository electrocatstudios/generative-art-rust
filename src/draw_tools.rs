use image::{RgbaImage, Rgba};

use crate::{WIDTH,HEIGHT};

pub struct Location<T> {
    pub x: T,
    pub y: T
}

pub fn draw_square(frame: &mut RgbaImage, color: Rgba<u8>, location: &Location<u32>, size: u32) {
    for x in (location.x - (size/2))..(location.x + (size/2)) {
        for y in (location.y - (size/2))..(location.y + (size/2)) {
            if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
                frame.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_circle(frame: &mut RgbaImage, color: Rgba<u8>, location: &Location<u32>, size: u32) {
    for x in (location.x - (size/2))..(location.x + (size/2)) {
        for y in (location.y - (size/2))..(location.y + (size/2)) {
            if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
                let x_diff = (x as i32 - location.x as i32).abs();
                let y_diff = (y as i32 - location.y as i32).abs();
                if ( (x_diff*x_diff) as f32 + (y_diff * y_diff) as f32 ).sqrt() <= (size/2) as f32 {
                    frame.put_pixel(x, y, color);
                }
            }
        }
    }
}
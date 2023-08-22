use rand::prelude::*;
use image::{RgbaImage, Rgba};
use std::vec; 

use crate::{WIDTH,HEIGHT};

pub struct State {}

pub fn get_initial_state() -> State {
    let ret = State{};
    ret
}

pub fn render_frame(frame: &mut RgbaImage, fraction: f32, state: &mut State) {
    if fraction < 0.1 {
        for x in 0..30 {
            for y in 0..30 {
                frame.put_pixel(x + 100, y + 100, Rgba([55,255,255,255]));
            }
        }
    }   
}
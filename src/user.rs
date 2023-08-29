use rand::prelude::*;
use image::{RgbaImage, Rgba};
use std::vec; 

use crate::draw_tools::*;
use crate::{WIDTH,HEIGHT};

const SIZE: u32 = 15;

pub struct State {}

pub fn get_initial_state() -> State {
    let ret = State{};
    ret
}

pub fn render_frame(frame: &mut RgbaImage, fraction: f32, state: &mut State) {
    if fraction / 1.5 < 0.85 { 
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(SIZE..(WIDTH-SIZE));
        let y = rng.gen_range(SIZE..(HEIGHT-SIZE));
        let loc = Location {
            x: x,
            y: y
        };
        let red: u8 = rng.gen_range(85..205);
        let green: u8 = rng.gen_range(85..225);
        let blue: u8 = rng.gen_range(85..255);

        draw_circle(frame, Rgba([red,green,blue,255]), &loc, SIZE);
    }   
}
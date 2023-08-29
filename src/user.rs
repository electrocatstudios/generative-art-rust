use rand::prelude::*;
use image::{RgbaImage, Rgba};
use std::vec; 

use crate::draw_tools::*;
use crate::{WIDTH,HEIGHT};

pub struct State {}

pub fn get_initial_state() -> State {
    let ret = State{};
    ret
}

pub fn render_frame(frame: &mut RgbaImage, fraction: f32, state: &mut State) {
    draw_circle(
        frame, 
        Rgba([0,255,0,255]), 
        &Location{
            x:WIDTH/3,
            y:HEIGHT/2,
        }, 
        50
    );

    draw_square(
        frame, 
        Rgba([0,55,255,255]), 
        &Location{
            x:(WIDTH/3) * 2,
            y:(HEIGHT/2)
        }, 
        50
    );
}
use rand::prelude::*;
use image::{RgbImage, Rgb};
use std::vec; 

use crate::{WIDTH,HEIGHT};

pub struct State {}

pub fn get_initial_state() -> State {
    let ret = State{};
    ret
}

pub fn render_frame(frame: &mut RgbImage, fraction: f32, state: &mut State) {   
}
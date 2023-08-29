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
}
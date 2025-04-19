use rand::prelude::*;
use image::{RgbaImage, Rgba};
use std::os::macos::raw::stat;
use std::vec; 

use crate::draw_tools::*;
use crate::{WIDTH,HEIGHT};

pub struct State {}

pub fn get_initial_state() -> State {
    let ret = State{};
    ret
}

pub fn render_frame(frame: &mut RgbaImage, fraction: f32, state: &mut State) {


    render_hex(frame, fraction, state, (250, 250)); // Center point
    render_hex(frame, fraction, state, (-50, 250)); // Center point - L
    render_hex(frame, fraction, state, (550, 250)); // Center point - R
    render_hex(frame, fraction, state, (100, 340)); // Left
    render_hex(frame, fraction, state, (400, 340)); // Right

    // Below
    render_hex(frame, fraction, state, (250, 425)); // Center point below
    render_hex(frame, fraction, state, (-50, 425)); // Center point below - L
    render_hex(frame, fraction, state, (550, 425)); // Center point below - R
    render_hex(frame, fraction, state, (100, 515)); // Left
    render_hex(frame, fraction, state, (400, 515)); // Right

    // Above
    render_hex(frame, fraction, state, (250, 75)); // Center point below
    render_hex(frame, fraction, state, (-50, 75)); // Center point below -L
    render_hex(frame, fraction, state, (550, 75)); // Center point below -R
    render_hex(frame, fraction, state, (100, 165)); // Left
    render_hex(frame, fraction, state, (400, 165)); // Right
    render_hex(frame, fraction, state, (100, -8)); // Left
    render_hex(frame, fraction, state, (400, -8)); // Right
}

pub fn render_hex(frame: &mut RgbaImage, fraction: f32, state: &mut State, offset: (i32, i32) ) {
    // Normalise the fraction (goes between 0.5 and 1.5);
    let fraction = if fraction > 1.0 {
        fraction - 1.0
    } else {
        fraction
    };

    let added_rot = {
        (fraction * 6.).floor() * std::f32::consts::PI / 3.
    };
    let added_rot = (2. * std::f32::consts::PI) - added_rot;
    let fraction = (fraction * 6.) % 1.;

    let mut col = Rgba::from([255, 55, 0, 255]);
    let outline = Rgba::from([205, 105, 0, 255]);
    
    draw_triangle_flip(frame, outline, col, fraction, added_rot, offset);
    col[0] -= 30;
    col[1] += 20;
    draw_triangle_flip(frame, outline, col, fraction, added_rot + std::f32::consts::PI * (60. / 180.), offset);
    col[0] -= 30;
    col[1] += 20;
    draw_triangle_flip(frame, outline, col, fraction, added_rot + std::f32::consts::PI * (120. / 180.), offset);
    col[0] -= 30;
    col[1] += 20;
    draw_triangle_flip(frame, outline, col, fraction, added_rot + std::f32::consts::PI, offset);
    col[0] -= 30;
    col[1] += 20;
    draw_triangle_flip(frame, outline, col, fraction, added_rot + std::f32::consts::PI * (240. / 180.), offset);
    col[0] -= 30;
    col[1] += 20;
    draw_triangle_flip(frame, outline, col, fraction, added_rot + std::f32::consts::PI * (300. / 180.), offset);
    
}

fn draw_triangle_flip(frame: &mut RgbaImage, outline: Rgba<u8>, col: Rgba<u8>, fraction: f32, rot: f32, offset: (i32, i32)) {
    let hex_size = 100.;
    let center = (offset.0, offset.1);
    let pt1_start_rot = std::f32::consts::PI * (30./180.) + rot;
    let pt1_start = (pt1_start_rot.sin() * hex_size, pt1_start_rot.cos() * hex_size);
    let pt1_start = (pt1_start.0 + center.0 as f32, pt1_start.1 + center.1 as f32);
    
    let pt1_end_rot = 2.0 * std::f32::consts::PI * (270./360.) + rot;
    let pt1_end = (pt1_end_rot.sin() * hex_size, pt1_end_rot.cos() * hex_size);
    let pt1_end = (pt1_end.0 + center.0 as f32, pt1_end.1 + center.1 as f32);
    
    let pt1 = (
        ((1. - fraction) * pt1_start.0 + (pt1_end.0 * fraction)), 
        ((1. - fraction) * pt1_start.1 + (pt1_end.1 * fraction))
    );
    let pt1 = (pt1.0 as i32, pt1.1 as i32);

    let pt2_rot = 2.0 * std::f32::consts::PI * (330./360.) + rot;
    let pt2 = (pt2_rot.sin() * hex_size, pt2_rot.cos() * hex_size);
    let pt2 = (pt2.0 as i32 + center.0, pt2.1 as i32 + center.1);
    
    draw_filled_triangle_from_points(frame, outline, col, center, pt1, pt2);

}
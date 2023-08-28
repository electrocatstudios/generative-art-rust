use rand::prelude::*;
use image::{RgbaImage, Rgba};
use std::vec; 
use imageproc::drawing::draw_line_segment_mut;

use crate::{WIDTH,HEIGHT};

const NUMBER_POINTS: u32 = 120;
const NUMBER_BRANCHES: u32 = 25;
const MIN_LENGTH: u32 = 2;
const MAX_LENGTH: u32 = 8;
const NUMBER_UNIQUE_REPETITIONS: f32 = 3.0;

pub struct Location<T> {
    pub x: T,
    pub y: T,
}

pub struct BranchObject {
    pub point_list: vec::Vec::<Location<u32>>,
    pub color: Rgba<u8>
}

pub struct State {
    pub branch_list: vec::Vec::<vec::Vec::<BranchObject>>,
}

pub fn get_branch_list(center_x: u32, center_y: u32) -> vec::Vec::<BranchObject> {
    let mut rng = rand::thread_rng();
    let mut branches: vec::Vec::<BranchObject> = vec::Vec::new();

    for i in 0..NUMBER_BRANCHES {
        let mut vec_points: vec::Vec::<Location<u32>> = vec::Vec::new();
        vec_points.push(Location{x: center_x, y: center_y});
    
        let angle_fraction: f32 = (2.0*std::f32::consts::PI) / NUMBER_BRANCHES as f32;
        let angle: f32 = i as f32 * angle_fraction;
        let mut prev_x: i32 = center_x as i32;
        let mut prev_y: i32 = center_y as i32;

        for j in 0..NUMBER_POINTS {
            let angle_adjusted: f32 = if j >= 3 {angle+(std::f32::consts::PI*0.75)} else {angle+angle_fraction};
            let rot: f32 = rng.gen_range(angle..angle_adjusted);
            let size = rng.gen_range(MIN_LENGTH..MAX_LENGTH);
            prev_x = prev_x + (rot.sin() * size as f32) as i32;
            prev_y = prev_y + (rot.cos() * size as f32) as i32;
            vec_points.push(Location{x: prev_x as u32, y:prev_y as u32});
        }

        let red: u8 = rng.gen_range(155..255);
        let green :u8 = rng.gen_range(55..120);
        let blue: u8 = rng.gen_range(100..155);

        branches.push(BranchObject{
            point_list: vec_points,
            color: Rgba([red,green,blue,255])
        });
    }
    branches
}

pub fn get_initial_state() -> State {
    // Set up our vector of branch objects
    let center_x:u32 = WIDTH/2;
    let center_y:u32 = HEIGHT/2;

    let mut branch_list: vec::Vec::<vec::Vec::<BranchObject>> = vec::Vec::new();
    for _ in 0..(NUMBER_UNIQUE_REPETITIONS as u32) {
        branch_list.push(get_branch_list(center_x, center_y));
    }
    let ret = State{
        branch_list: branch_list,
    };
    ret
}

pub fn render_frame(frame: &mut RgbaImage, fraction: f32, state: &mut State) {
    let frame_fraction = if fraction <= 1.0 {fraction} else {fraction - 1.0};

    let mut branch_list = &state.branch_list;
    let frame_fraction_single = 1.0/NUMBER_UNIQUE_REPETITIONS;
    let branch_num: usize = (frame_fraction/frame_fraction_single) as usize;
    let mut line_count = ((frame_fraction - (frame_fraction_single * branch_num as f32)) * NUMBER_POINTS as f32) as u32;
    
    if line_count >= NUMBER_POINTS {
        line_count = NUMBER_POINTS - 1;
    }

    if branch_num < state.branch_list.len() {
        for branch in state.branch_list[branch_num].iter() {
            for i in 1..line_count+1{
                let start: Location<f32> = Location{
                    x: branch.point_list[(i-1) as usize].x as f32,
                    y: branch.point_list[(i-1) as usize].y as f32
                };
                let end: Location<f32> = Location{
                    x: branch.point_list[i as usize].x as f32,
                    y: branch.point_list[i as usize].y as f32
                };
    
                if end.x < WIDTH as f32 && end.x >= 0.0 && end.y < HEIGHT as f32 && end.y >= 0.0 {
                    draw_line_segment_mut(
                        frame, 
                        (start.x, start.y), 
                        (end.x, end.y), 
                        branch.color
                    );
                }
                
            }
        }  
    }
}
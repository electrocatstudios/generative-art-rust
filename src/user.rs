use rand::prelude::*;
use image::{RgbImage, Rgb};
use std::vec; 

use crate::{WIDTH,HEIGHT};

const NUM_VORTICES: u32 = 5;

struct Location<T> {
    x: T,
    y: T
}

struct VortexPoint {
    center: Location<u32>,
    color: Rgb<u8>,
    rotation: f32,
    size: u32,
    width: u32
}

pub struct State {
    vortices: vec::Vec::<VortexPoint>
}

pub fn get_initial_state() -> State {
    
    let mut vortices: Vec::<VortexPoint> = Vec::new();
    let center = Location{x: WIDTH/2, y:HEIGHT/2};
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_VORTICES {
        let diff_x: i32 = rng.gen_range(-10..10);
        let diff_y: i32 = rng.gen_range(-10..10);
        let red: u8 = rng.gen_range(55..255);
        let green: u8 = rng.gen_range(55..255);
        let blue: u8 = rng.gen_range(55..255);
        let rot: f32 = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let circle_size: u32 =  rng.gen_range(40..200);

        let nxt = VortexPoint{
            center: Location{x: (center.x as i32 + diff_x) as u32, y: (center.y as i32 + diff_y) as u32},
            color: Rgb([red,green,blue]),
            rotation: rot,
            size: 5,
            width: circle_size
        };
        vortices.push(nxt);
    }
    let ret = State{vortices: vortices};
    ret
}

pub fn render_frame(frame: &mut RgbImage, fraction: f32, state: &mut State) {
   
    let frame_fraction_radian = fraction * 2.0 * std::f32::consts::PI;

    for vp in state.vortices.iter_mut() {
        let half_size = vp.size/2;
        for x in 0..vp.size {
            for y in 0..vp.size {
                let center_x: i32 = vp.center.x as i32 - half_size as i32 + x as i32;
                let rot_x = (frame_fraction_radian + vp.rotation).sin() ;
                let pos_x = center_x + (rot_x * (vp.width as f32)) as i32;
                
                let center_y: i32 = vp.center.x as i32 - half_size as i32 + y as i32;
                let rot_y = (frame_fraction_radian + vp.rotation).cos();
                let pos_y = center_y + (rot_y * (vp.width as f32)) as i32;

                if pos_x >= 0 && pos_x < WIDTH as i32 && pos_y >= 0 && pos_y < HEIGHT as i32 {
                    frame.put_pixel(pos_x as u32,pos_y as u32,vp.color);
                }
            }
        }
    }

}
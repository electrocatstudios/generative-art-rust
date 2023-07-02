// extern crate image;
use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};
use std::fs::File;
use std::fs;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 60;
const SIZE: u32 = 16;

fn main() {
    println!("Starting");

    // Generate folders if missing
    fs::create_dir_all("gifs").expect("Failed to generate gifs folder");
    fs::create_dir_all("outputs").expect("Failed to generate outputs folder");

    let color_map = &[];//&[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = File::create("gifs/output.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    for frame in 0..FRAMES {
        // a default (black) image containing Rgb values
        let mut image: RgbImage = RgbImage::new(WIDTH, HEIGHT);
        // Set background
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                image.put_pixel(x, y, Rgb([120, 120, 120]));
            }
        }


        let frame_fraction = frame as f32 / (WIDTH as f32/FRAMES as f32);
        
        let mut offset_x: u32 = 0;
        
        if frame != 0 {
            offset_x = frame * WIDTH/FRAMES;
        }
        let half_width: f32 = HEIGHT as f32/2.0 ;
        let offset_y: i32 = (HEIGHT as i32/2) - (frame_fraction.sin() * half_width) as i32;
        // print!("{:?} -> {:?} \n", offset_y, frame_fraction.sin());
        
        // set a central pixel to white
        for x in 0..SIZE {
            for y in 0..SIZE {
                let x_pos = x + offset_x;
                let y_pos = y + offset_y as u32;

                if x_pos > 0 && x_pos < WIDTH && y_pos > 0 && y_pos < HEIGHT {
                    image.put_pixel(x_pos, y_pos, Rgb([55, 255, 55]));
                }
            
            }
        }

        let mut frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, image.as_raw());
        frame.delay = 1;
        encoder.write_frame(&frame).unwrap();
        // write it out to a file
        // let filename = "outputs/".to_owned() + &frame.to_string() + ".png";
        // image.save(filename).unwrap();
    }

    println!("Done");


}
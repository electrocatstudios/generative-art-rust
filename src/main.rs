use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};

use minimp4;
use openh264;
use rand::prelude::*;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fs;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 240;
const REPETITIONS: u32 = 3;
const NUM_VORTICES: u32 = 40;
const DECAY: u32 = 4;

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

fn main() {
    println!("Starting");

    // Generate output folder if missing
    fs::create_dir_all("outputs").expect("Failed to generate outputs folder");
    // End generate output folders

    // GIF setup
    let color_map = &[];
    let mut image = fs::File::create("outputs/output.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    // Video setup    
    let config = openh264::encoder::EncoderConfig::new(WIDTH, HEIGHT);
    let mut vid_encoder = openh264::encoder::Encoder::with_config(config).unwrap();

    // Generate the image - store prev frame in prev_image
    let mut prev_image: Option<RgbImage> = None;
    let mut buf = Vec::new();

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

    for frame in 0..FRAMES*2 {
        let mut image: RgbImage = RgbImage::new(WIDTH, HEIGHT);

        // Copy previous image or create new
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                match prev_image {
                    Some(ref prev_image) => {
                        let mut pix_col: Rgb<u8> = *prev_image.get_pixel(x,y);
                        // Add decay to any non-black pixel
                        if pix_col[0] != 0 {
                            if pix_col[0] >= DECAY as u8 {
                                pix_col[0] -= DECAY as u8;
                            }else{
                                pix_col[0] = 0;
                            }
                        }
                        if pix_col[1] != 0 {
                            if pix_col[1] >= DECAY as u8 {
                                pix_col[1] -= DECAY as u8;
                            }else{
                                pix_col[1] = 0;
                            }
                        }
                        if pix_col[2] != 0 {
                            if pix_col[2] >= DECAY as u8 {
                                pix_col[2] -= DECAY as u8;
                            }else{
                                pix_col[2] = 0;
                            }
                        }
                        image.put_pixel(x,y,pix_col);
                    },
                    None => {
                        image.put_pixel(x, y, Rgb([0, 0, 0]));
                    }
                }
            }
        }

        // Perform caluclation of current frame state
        let frame_fraction = frame as f32 / FRAMES as f32;
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0); // Multiplied by 50 because we render rounds of the 
                                                                     // animation loop, to catch any decaying pixels
        //// #### TODO: Put your code here
        let frame_fraction_radian = frame_fraction * 2.0 * std::f32::consts::PI;

        for vp in vortices.iter_mut() {
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
                        image.put_pixel(pos_x as u32,pos_y as u32,vp.color);
                    }
                }
            }
        }
        // End frame generation

        // Generate next frame for output - we run the animation twice and capture the second half of the first reptition
        // and the first half of the second repetition - meaning we can get any fading colors captured as well
        if frame >= FRAMES / 2 && frame < (FRAMES*3/2) {
            // GIF 
            let mut frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, image.as_raw());
            frame.delay = 1;
            encoder.write_frame(&frame.clone()).unwrap();

            // MP4
            let yuv = openh264::formats::YUVBuffer::with_rgb(WIDTH as usize, HEIGHT as usize,&image.as_raw());
            let bitstream = vid_encoder.encode(&yuv).unwrap();
            bitstream.write_vec(&mut buf);
        }
        
        prev_image = Some(image.to_owned());
    }
    
    print!("\rRendering Frames: 100%-----\n"); // dashes to overwrite prev line
    println!("Rendering video output from buffer");

    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = minimp4::Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(WIDTH as i32, HEIGHT as i32, false, "Generated Video");
    for _ in 0..REPETITIONS {
        mp4muxer.write_video(&buf);
    }
    mp4muxer.close();

    let mut video_bytes = Vec::new();
    video_buffer.seek(SeekFrom::Start(0)).unwrap();    
    video_buffer.read_to_end(&mut video_bytes).unwrap();
    
    std::fs::write("outputs/output.mp4", &video_bytes).unwrap();

    println!("Done rendering video output");

    println!("Done");
    
}
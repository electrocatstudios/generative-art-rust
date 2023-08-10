use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};
use rand::prelude::*;

use minimp4;
use openh264;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fs;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 200;
const FRAMES: u32 = 100;
const REPETITIONS: u32 = 10;
const SIZE: u32 = 5;
const DECAY: u32 = 4;

struct ColorPoint {
    color: Rgb<u8>,
    height: u32,
    multiplier: u32
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

    let mut droplets = Vec::<ColorPoint>::new();
    let number_droplets = WIDTH/SIZE;
    let mut rng = rand::thread_rng();
    for _ in 0..number_droplets{
        let rand: f64 = rng.gen();
        let height: u32 = (HEIGHT as f64 * rand) as u32;
        let red = (rng.gen::<f64>() * 200.0) as u8 + 55;
        let green = (rng.gen::<f64>() * 200.0) as u8 + 55;
        let blue = (rng.gen::<f64>() * 200.0) as u8 + 55;
        let multi = (rng.gen::<f64>() * 2.0) as u32 + 1;
        droplets.push(ColorPoint{color:Rgb([red,green,blue]), height: height, multiplier: multi});
    }

    let fall_speed = HEIGHT as f64 / FRAMES as f64;

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
        print!("Rendering Frames: {:.2}%\n", frame_fraction * 50.0); // Multiplied by 50 because we render rounds of the 
                                                                     // animation loop, to catch any decaying pixels
        //// #### TODO: Put your code here
        let mut count = 0;
        for col_pt in droplets.iter_mut() {
            col_pt.height += (fall_speed as u32) * col_pt.multiplier;
            if col_pt.height > HEIGHT {
                col_pt.height -= HEIGHT;
            }
            let x_pos = count * SIZE;
            for x in x_pos..(x_pos+SIZE){
                for y in col_pt.height..(col_pt.height+SIZE){
                    if x < WIDTH && y<HEIGHT {
                        image.put_pixel(x, y, col_pt.color);
                    }
                }
            }
            count += 1;
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
        
        prev_image = Some(image.clone());
    }
    
    print!("\rRendering Frames: 100%-----\n"); // dashes to overwrite prev line
    println!("Rendering video output from buffer");

    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = minimp4::Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(WIDTH as i32, HEIGHT as i32, false, "Moving circle.");
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
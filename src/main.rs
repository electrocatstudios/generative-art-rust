// extern crate image;
use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};
use imageproc::drawing::draw_line_segment_mut;

use minimp4;
use openh264;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fs;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 120;
const REPETITIONS: u32 = 10;
const SIZE: u32 = 64;
const DECAY: u32 = 4;

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
        let mut frame_fraction = frame as f32 / FRAMES as f32;    
        if frame_fraction > 1.0 {
            // Because we go around twice (so second frame run will be > 1)
            frame_fraction -= 1.0;
        }
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0); // Multiplied by 50 because we render rounds of the 
                                                                     // animation loop, to catch any decaying pixels
        
        let frame_fraction_radians = frame_fraction * 2.0 * std::f32::consts::PI;
        let green: u8 = (frame_fraction_radians.cos() * 255.0) as u8;
        let blue: u8 = (frame_fraction_radians.sin() * 255.0) as u8;
        let color = Rgb([100, green, blue]);

        let center_x: f32 = WIDTH as f32 / 2.0;
        let center_y: f32 = HEIGHT as f32 / 2.0;
        let line_point_x = center_x * frame_fraction_radians.sin();
        let line_point_y = center_y * frame_fraction_radians.cos();
    
        draw_line_segment_mut(&mut image, (center_x + line_point_x, center_y + line_point_y ), (center_x, center_y), color);
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
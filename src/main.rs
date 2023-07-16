// extern crate image;
use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};
use imageproc::drawing::draw_line_segment_mut;

use minimp4;
use openh264;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fs;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 200;
const FRAMES: u32 = 120;
const REPETITIONS: u32 = 10;
// const SIZE: u32 = 128;
const DECAY: u32 = 15;

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
            frame_fraction -= 1.0;
        }
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0); // Multiplied by 50 because we render rounds of the 
                                                                     // animation loop, to catch any decaying pixels

        let box_size: f32 = 50.0;
        let number_x = WIDTH/box_size as u32;
        let number_y = HEIGHT/box_size as u32;
        for tl_x in 0..number_x {
            for tl_y in 0..number_y {
                let color_frac = (tl_x + tl_y) as f32 / (number_x + number_y) as f32;
            
                let red = 128;
                let blue_diff: u8 = (127.0 * color_frac) as u8;

                let green = 128 + blue_diff;
                let blue: u8 = 255 - blue_diff ;
                // println!("Color frac {:?} -> {:?} ====> {:?}", color_frac, blue_diff, blue);

                // let green = 128 + (127.0 * color_frac) as u8;
                // let blue = 255 - (127.0 * color_frac) as u8;
                let color = Rgb([red, green, blue]);
                
                let top_left_x: f32 = tl_x as f32 * box_size;
                let top_left_y: f32 = tl_y as f32 * box_size;
                let angle_fraction = frame_fraction * (std::f32::consts::PI * 2.0);

                // Border
                // draw_line_segment_mut(&mut image, (top_left_x, top_left_y), (top_left_x + box_size, top_left_y), color);
                // draw_line_segment_mut(&mut image, (top_left_x + box_size, top_left_y), (top_left_x + box_size, top_left_y + box_size), color);
                // draw_line_segment_mut(&mut image, (top_left_x + box_size, top_left_y + box_size), (top_left_x, top_left_y + box_size), color);
                // draw_line_segment_mut(&mut image, (top_left_x, top_left_y + box_size), (top_left_x, top_left_y), color);

                if frame_fraction < 0.25 {
                    let start_x = top_left_x;
                    let start_y = top_left_y;
                    let end_x = start_x + (angle_fraction.sin() * box_size);
                    let end_y = start_y + (angle_fraction.cos() * box_size);
                    draw_line_segment_mut(&mut image, (start_x, start_y), (end_x, end_y), color);
                    
                } else if frame_fraction < 0.5 {
                    let start_x = top_left_x + box_size;
                    let start_y = top_left_y;
                    let end_x = start_x - (angle_fraction.sin() * box_size);
                    let end_y = start_y - (angle_fraction.cos() * box_size);
                    draw_line_segment_mut(&mut image, (start_x, start_y), (end_x, end_y), color);
                    
                } else if frame_fraction < 0.75 {
                    let start_x = top_left_x + box_size;
                    let start_y = top_left_y + box_size;
                    let end_x = start_x + (angle_fraction.sin() * box_size);
                    let end_y = start_y + (angle_fraction.cos() * box_size);
                    draw_line_segment_mut(&mut image, (start_x, start_y), (end_x, end_y), color);
                    
                } else {
                    let start_x = top_left_x;
                    let start_y = top_left_y + box_size;
                    let end_x = start_x + (angle_fraction.sin() * -box_size);
                    let end_y = start_y - (angle_fraction.cos() * box_size);
                    draw_line_segment_mut(&mut image, (start_x, start_y), (end_x, end_y), color);
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
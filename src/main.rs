// extern crate image;
use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};

use minimp4;
use openh264;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fs;
use std::path::PathBuf;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 120;
const REPETITIONS: u32 = 10;
const SIZE: u32 = 128;
const DECAY: u32 = 4;

fn main() {
    println!("Starting");

    // Generate output folders if missing
    fs::create_dir_all("gifs").expect("Failed to generate gifs folder");
    // fs::create_dir_all("outputs").expect("Failed to generate outputs folder");
    fs::create_dir_all("videos").expect("Failed to generate videos folder");
    // End generate output folders

    // GIF setup
    let color_map = &[];//&[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = fs::File::create("gifs/output.gif").unwrap();
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
        let mut fr = frame;
        if fr >= FRAMES {
            fr -= FRAMES;
        }
        let frame_fraction = frame as f32 / FRAMES as f32;
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0);

        let fraction_radian = frame_fraction * (std::f32::consts::PI * 2.0);
        let half_width: f32 = WIDTH as f32/2.0;
        let offset_x =  (WIDTH as i32/2) + (fraction_radian.sin() * (half_width * 0.8)) as i32;
        let half_height: f32 = HEIGHT as f32/2.0 ;
        let offset_y: i32 = (HEIGHT as i32/2) - (fraction_radian.cos() * (half_height * 0.8)) as i32;
        let red:u8 = 100 + (fraction_radian.sin() * 155.0) as u8;
        let green: u8 = 255 -(fraction_radian.sin() * 255.0) as u8 ;
        let blue: u8 = 255 - (fraction_radian.cos() * 255.0) as u8; 
        // set a central pixel to white
        for x in 0..SIZE {
            for y in 0..SIZE {
                let half_size = SIZE as i32/2;
                let x_pos = (x as i32 + offset_x) - half_size;
                let y_pos = (y as i32 + offset_y) - half_size;

                if x_pos > 0 && x_pos < WIDTH as i32 && y_pos > 0 && y_pos < HEIGHT as i32 {
                    image.put_pixel(x_pos as u32, y_pos as u32, Rgb([red, green, blue]));
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
    
    std::fs::write("videos/output.mp4", &video_bytes).unwrap();

    println!("Done rendering video output");

    println!("Done");
    

}
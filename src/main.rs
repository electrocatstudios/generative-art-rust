
use image::{RgbaImage,Rgba};

#[cfg(not(target_arch="wasm32"))]
use gif::{Frame, Encoder, Repeat};
#[cfg(not(target_arch="wasm32"))]
use minimp4;
#[cfg(not(target_arch="wasm32"))]
use openh264;
#[cfg(not(target_arch="wasm32"))]
use std::io::{Cursor, Read, Seek, SeekFrom};
#[cfg(not(target_arch="wasm32"))]
use std::fs;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch="wasm32")]
use web_time::SystemTime;
#[cfg(target_arch="wasm32")]
use std::cell::RefCell;
#[cfg(target_arch="wasm32")]
use std::rc::Rc;

mod user;
mod utils;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
const FRAMES: u32 = 360;
const FRAMES_PER_SECOND: f32 = 10.0;
const REPETITIONS: u32 = 10;

const DECAY: u32 = 6;

fn prepare_next_frame(prev_image: Option<RgbaImage>, frame_count: u8) -> RgbaImage {
    let mut image: RgbaImage = RgbaImage::new(WIDTH, HEIGHT);
    // Copy previous image or create new
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            match prev_image {
                Some(ref prev_image) => {
                    let mut pix_col: Rgba<u8> = *prev_image.get_pixel(x,y);
                    // Add decay to any non-black pixel
                    if pix_col[0] != 0 {
                        if pix_col[0] >= frame_count * DECAY as u8 {
                            pix_col[0] -= frame_count * DECAY as u8;
                        }else{
                            pix_col[0] = 0;
                        }
                    }
                    if pix_col[1] != 0 {
                        if pix_col[1] >= frame_count * DECAY as u8 {
                            pix_col[1] -= frame_count * DECAY as u8;
                        }else{
                            pix_col[1] = 0;
                        }
                    }
                    if pix_col[2] != 0 {
                        if pix_col[2] >= frame_count * DECAY as u8 {
                            pix_col[2] -= frame_count * DECAY as u8;
                        }else{
                            pix_col[2] = 0;
                        }
                    }
                    image.put_pixel(x,y,Rgba([pix_col[0],pix_col[1],pix_col[2],255]));
                },
                None => {
                    image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
                }
            }
        }
    }
    image
}

#[cfg(not(target_arch="wasm32"))]
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
    let mut prev_image: Option<RgbaImage> = None;
    let mut buf = Vec::new();

    let mut state = user::get_initial_state();

    for frame in 0..(3*FRAMES/2) {
        let mut image: RgbaImage = prepare_next_frame(prev_image, 1); // Always one because we are rendering each frame at the same time

        // Perform caluclation of current frame state
        let frame_fraction = frame as f32 / FRAMES as f32;
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0); // Multiplied by 50 because we render rounds of the 
                                                                     // animation loop, to catch any decaying pixels
 
        user::render_frame(&mut image, frame_fraction, &mut state);
 
        // Generate next frame for output - we run the animation twice and capture the second half of the first reptition
        // and the first half of the second repetition - meaning we can get any fading colors captured as well
        if frame >= FRAMES / 2 && frame < (FRAMES*3/2) {
            // GIF 
            let mut frame = Frame::from_rgba(WIDTH as u16, HEIGHT as u16, &mut image.clone().into_raw());
            frame.delay = 1;
            encoder.write_frame(&frame.clone()).unwrap();

            // MP4
            // for _ in 0..5 {
            let yuv = openh264::formats::YUVBuffer::with_rgb(WIDTH as usize, HEIGHT as usize,&utils::rgba8_to_rgb8(image.clone()).as_raw());
            let bitstream = vid_encoder.encode(&yuv).unwrap();
            bitstream.write_vec(&mut buf);
            // }
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


#[cfg(target_arch="wasm32")]
fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut prev_image: Option<RgbaImage> = None;
    let mut state = user::get_initial_state();
    let mut prev_time = SystemTime::now();
    let mut cur_frame = 0;
 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Main render loop
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let diff = SystemTime::now().duration_since(prev_time).expect("Failed to calculate previous difference").as_millis() as f32 / 1000.0;
        if diff > 1.0 {
            // More than 1 second has elapsed since the previous animation frame
            // Possible that we lost focus on the window
            web_sys::console::log_1(&"Overtime warning".into());
        }
 
        let frame_plus: u32 = (diff * FRAMES_PER_SECOND as f32) as u32; 
        if frame_plus >= 1 {
            if frame_plus > 1 {
                web_sys::console::log_1(&"over-framed".into());
            }
            let mut image: RgbaImage = prepare_next_frame(prev_image.to_owned(), frame_plus as u8);
            
            cur_frame += frame_plus;
            prev_time = SystemTime::now();
            if cur_frame > FRAMES {
                cur_frame -= FRAMES;
            }

            let frame_fraction: f32 = cur_frame as f32 / FRAMES as f32;

            user::render_frame(&mut image, frame_fraction, &mut state);
        
            let clamped_buf: wasm_bindgen::Clamped<&[u8]> = wasm_bindgen::Clamped(image.as_raw());        
            let image_data_temp = match web_sys::ImageData::new_with_u8_clamped_array(clamped_buf, WIDTH) {
                Ok(res) => {
                    res
                },
                Err(err) => {
                    web_sys::console::log_1(&"Err while creating image data temp".into());
                    web_sys::console::log_1(&err);
                    web_sys::ImageData::new_with_sw(WIDTH, HEIGHT).unwrap()
                }
            };
            
            prev_image = Some(image.to_owned());

            let res = context.put_image_data(&image_data_temp, 0.0, 0.0);
            match res {
                Ok(_) => {},
                Err(jsval) => web_sys::console::log_1(&jsval)
            }
        } 

        // Schedule ourself for another requestAnimationFrame callback.
        utils::request_animation_frame(f.borrow().as_ref().unwrap());
      
    }) as Box<dyn FnMut()>));

    utils::request_animation_frame(g.borrow().as_ref().unwrap());
}
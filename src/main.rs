// extern crate image;
use gif::{Frame, Encoder, Repeat};
use image::{RgbImage,Rgb};
use imageproc::drawing::draw_text_mut;

use minimp4;
use openh264;
use rusttype::{Font,Scale}; 

use rand::Rng;

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::{fs,vec};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 100;
const REPETITIONS: u32 = 10;
const SIZE: u32 = 128;

const RED_DECAY: u32 = 6;
const GREEN_DECAY: u32 = 4;
const BLUE_DECAY: u32 = 5;

const NUMBER_CHARACTERS_PER_LANE: u32 = 24;
const NUMBER_LANES: u32 = 18;
const BLUR: u32 = 5;

fn get_font() -> Font<'static> {
    let font_data: &[u8] = include_bytes!("../fonts/fira-sans.bold.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("Error getting font");
    font
}

struct LetterDrop {
    x: i32,
    offset: usize,
    letters: vec::Vec::<String>,
    cur_sel: usize
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

    let font = get_font();
    
   
    let mut rng = rand::thread_rng();

    let mut letter_drops: vec::Vec::<LetterDrop> = vec::Vec::new();
    let lane_width = (WIDTH - 10)/NUMBER_LANES; // 10 is twice the border width

    for lane_num in 0..NUMBER_LANES {
        let mut vec_letters = vec::Vec::new();
        for i in 0..NUMBER_CHARACTERS_PER_LANE{
            let letter: char = rng.gen_range(b'A'..b'Z') as char;
            let s = format!("{}", letter);
            vec_letters.push(s);
        }
       
        let offset: usize = rng.gen_range(0..NUMBER_CHARACTERS_PER_LANE-1) as usize;
        let mut letter_drop = LetterDrop{x: (lane_num * lane_width) as i32 + 5, offset: offset as usize, letters:vec_letters, cur_sel:0};
        letter_drops.push(letter_drop);
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
                            if pix_col[0] >= RED_DECAY as u8 {
                                pix_col[0] -= RED_DECAY as u8;
                            }else{
                                pix_col[0] = 0;
                            }
                        }
                        if pix_col[1] != 0 {
                            if pix_col[1] >= GREEN_DECAY as u8 {
                                pix_col[1] -= GREEN_DECAY as u8;
                            }else{
                                pix_col[1] = 0;
                            }
                        }
                        if pix_col[2] != 0 {
                            if pix_col[2] >= BLUE_DECAY as u8 {
                                pix_col[2] -= BLUE_DECAY as u8;
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
        let mut fraction = frame as f32/FRAMES as f32;
        if fraction > 1.0 {
            fraction -= 1.0;
        }
        
        for letter_drop in letter_drops.iter_mut() {
            let cur_sel_point = letter_drop.letters.len() as f32 * fraction;
            
            letter_drop.cur_sel = cur_sel_point as usize + letter_drop.offset;
            if letter_drop.cur_sel > letter_drop.letters.len() - 1 {
                letter_drop.cur_sel -= letter_drop.letters.len();
            }
            
            let cycle_pos_fraction = letter_drop.cur_sel  as f32 / letter_drop.letters.len() as f32;
            let y_pos = (HEIGHT as f32 * cycle_pos_fraction) as i32; 
            let letter_height = HEIGHT/letter_drop.letters.len() as u32;
            
            let _ = draw_text_mut(
                    &mut image, 
                    Rgb([255,255,255]), 
                    letter_drop.x, 
                    y_pos, 
                    Scale { x: 32.0, y: 32.0 },
                    &font, 
                    &letter_drop.letters[letter_drop.cur_sel].clone()
                );
        }

        // Add the blur
        // for x in 0..WIDTH {
        //     for y in 0..HEIGHT {
                
        //         let mut pix_col = image.get_pixel(x,y);
        //         if pix_col[0] == 255 && pix_col[1] == 255 && pix_col[2] == 255 {
        //             // We have a white pixel
        //             for offset_x in BLUR..0 {
        //                 for offset_y in BLUR..0 {
        //                     // Add blur up and left
        //                     if x-offset_x >= 0  && y-offset_y >= 0 {
        //                         let mut check_pix= image.get_pixel(x-offset_x, y-offset_y);
        //                         if check_pix[0] == 0 && check_pix[1] == 0 && check_pix[2] == 0 {
        //                             // Empty pixel - calculate blue amount
        //                             let fade_amount = ((BLUR as f32 * 2.0) - (offset_x + offset_y) as f32) / (BLUR as f32 * 2.0);

        //                             let pix_val: u8 = (255.0 * fade_amount) as u8;
        //                             image.put_pixel(x-offset_x, y-offset_y, Rgb([pix_val, pix_val, pix_val]));

        //                         }
        //                     }
        //                 }
        //             }
        //             for offset_x in 0..BLUR {
        //                 for offset_y in 0..BLUR {
        //                     // Add blur down and right
        //                     if x+offset_x < WIDTH && y+offset_y < HEIGHT {
        //                         // println!("x:{:?}, y:{:?}", x+offset_x, y+offset_y);
        //                         let mut check_pix= image.get_pixel(x+offset_x, y+offset_y);
        //                         if check_pix[0] == 0 && check_pix[1] == 0 && check_pix[2] == 0 {
        //                             // Empty pixel - calculate blur amount
        //                             let fade_amount = (offset_x + offset_y) as f32 / (BLUR as f32 * 2.0);

        //                             let pix_val: u8 = (255.0 * fade_amount) as u8;
        //                             image.put_pixel(x+offset_x, y+offset_y, Rgb([pix_val, pix_val, pix_val]));

        //                         }
        //                     }
        //                 }
        //             }
        //         }
                
        //     }
        // }
        // END BLUR
        
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

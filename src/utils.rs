#[cfg(not(target_arch="wasm32"))]
use image::{Rgba, Rgb};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch="wasm32"))]
pub fn rgba8_to_rgb8(input: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;
    
    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();
    
    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];
    
    let mut i = 0;
    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for chunk in input.chunks(4) {
        // ... and copy each of them to output, leaving out the A byte
        output_data[i..i+3].copy_from_slice(&chunk[0..3]);
        i+=3;
    }
    
    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}

#[cfg(target_arch="wasm32")]
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window =  web_sys::window().expect("no global `window` exists");
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
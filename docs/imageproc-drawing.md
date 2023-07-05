# Color wheel (using imageproc)

![Example output imageproc color wheel](imageproc-drawing.gif)

This can be produced with the following code (also available on the the branch `example/imageproc-drawing`):

```rust
use imageproc::drawing::draw_line_segment_mut;

...snip

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 120;
const REPETITIONS: u32 = 10;
const SIZE: u32 = 64;
const DECAY: u32 = 4;

...snip

        // Perform caluclation of current frame state
        let frame_fraction = frame as f32 / FRAMES as f32;
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0);
        
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

```
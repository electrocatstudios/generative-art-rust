# Green Cycle Clockwise

![Example output rotating green square](green_cycle_clockwise.gif)

This can be produced with the following code (also available on the the branch `example/green-clockwise`):

```rust
// Constants at the top of the file
const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const FRAMES: u32 = 60;
const REPETITIONS: u32 = 3;
const SIZE: u32 = 16;
const DECAY: u32 = 10;

...snip

        // Perform caluclation of current frame state
        let frame_fraction = frame as f32 / FRAMES as f32;
        print!("\rRendering Frames: {:.2}%", frame_fraction * 50.0);
        
        let fraction_radian = frame_fraction * (std::f32::consts::PI * 2.0);
        let half_width: f32 = WIDTH as f32/2.0;
        let offset_x =  (WIDTH as i32/2) + (fraction_radian.sin() * (half_width * 0.8)) as i32;
        let half_height: f32 = HEIGHT as f32/2.0 ;
        let offset_y: i32 = (HEIGHT as i32/2) - (fraction_radian.cos() * (half_height * 0.8)) as i32;
        
        // Draws the square
        for x in 0..SIZE {
            for y in 0..SIZE {
                let x_pos = x + offset_x as u32;
                let y_pos = y + offset_y as u32;

                if x_pos > 0 && x_pos < WIDTH && y_pos > 0 && y_pos < HEIGHT {
                    image.put_pixel(x_pos, y_pos, Rgb([55, 255, 55]));
                }
            
            }
        }
        // End frame generation

```
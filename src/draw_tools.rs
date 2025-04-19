use image::{RgbaImage, Rgba};
use line_drawing::Bresenham;

use crate::{WIDTH,HEIGHT};

pub struct Location<T> {
    pub x: T,
    pub y: T
}

pub fn draw_square(frame: &mut RgbaImage, color: Rgba<u8>, location: &Location<u32>, size: u32) {
    for x in (location.x - (size/2))..(location.x + (size/2)) {
        for y in (location.y - (size/2))..(location.y + (size/2)) {
            if x < WIDTH && y < HEIGHT {
                frame.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_circle(frame: &mut RgbaImage, color: Rgba<u8>, location: &Location<u32>, size: u32) {
    for x in (location.x - (size/2))..(location.x + (size/2)) {
        for y in (location.y - (size/2))..(location.y + (size/2)) {
            if x < WIDTH && y < HEIGHT {
                let x_diff = (x as i32 - location.x as i32).abs();
                let y_diff = (y as i32 - location.y as i32).abs();
                if ( (x_diff*x_diff) as f32 + (y_diff * y_diff) as f32 ).sqrt() <= (size/2) as f32 {
                    // Bounds check
                    if x < WIDTH.try_into().unwrap() && y < HEIGHT.try_into().unwrap(){
                        frame.put_pixel(x, y, color);
                    }
                }
            }
        }
    }
}

pub fn draw_line(frame: &mut RgbaImage, color: Rgba<u8>, start: (i32, i32), end: (i32, i32)) {
    let ln: Bresenham<i32> = Bresenham::new(start, end);
    for (x,y) in ln {
        if x >= 0 && x < WIDTH.try_into().unwrap() && y >= 0 && y < HEIGHT.try_into().unwrap(){
            frame.put_pixel(x as u32, y as u32, color);
        }
    }
}

/// Draws the outline of a triangle (no fill)
pub fn draw_triangle_from_points(frame: &mut RgbaImage, col: Rgba<u8>, pt1: (i32, i32), pt2: (i32, i32), pt3: (i32, i32)) {
    draw_line(frame, col, pt1, pt2);
    draw_line(frame, col, pt1, pt3);
    draw_line(frame, col, pt2, pt3);
}

/// Draw filled triangel with given outline color
pub fn draw_filled_triangle_from_points(frame: &mut RgbaImage, col_outline: Rgba<u8>, col: Rgba<u8>, pt1: (i32, i32), pt2: (i32, i32), pt3: (i32, i32)) {
   
    // Do the fill
    // Get sorted list of points
    let mut points = [pt1, pt2, pt3];
    points.sort_by(|a, b| a.1.cmp(&b.1));

    let total_height = (points[2].1 - points[0].1) as f64;
    
    for y in (points[0].1)..=(points[1].1) {
        let segment_height = (points[1].1 - points[0].1) as f64;
        let alpha = (y - points[0].1) as f64 / total_height;
        let beta = (y - points[0].1) as f64 / segment_height;

        let left_point_x = points[0].0 as f64 + ((points[2].0 - points[0].0) as f64 * alpha);
        let left_point_x = if left_point_x < 0.0 {
            0.0
        } else {
            left_point_x
        };
        let right_point_x = points[0].0 as f64 + ((points[1].0 - points[0].0) as f64 * beta);
        let right_point_x = if right_point_x < 0.0 {
            0.0
        } else {
            right_point_x
        };

        draw_line(frame, col, (left_point_x as i32, y), (right_point_x as i32, y));
    }

    // draws the second "half" of the triangle
    for y in (points[1].1 as i32)..=(points[2].1 as i32) {

        let segment_height = (points[2].1 - points[1].1) as f64;
        let alpha = (y - points[0].1) as f64 / total_height;
        let beta = (y - points[1].1) as f64 / segment_height;

        let left_point_x = points[0].0 as f64 + ((points[2].0 - points[0].0) as f64 * alpha);
        let left_point_x = if left_point_x < 0.0 {
            0.0
        } else {
            left_point_x
        };
        let right_point_x = points[1].0 as f64 + ((points[2].0 - points[1].0) as f64 * beta);
        let right_point_x = if right_point_x < 0.0 {
            0.0
        } else {
            right_point_x
        };
        draw_line(frame, col, (left_point_x as i32, y), (right_point_x as i32, y));
    
        // Outline last
        draw_line(frame, col_outline, pt1, pt2);
        draw_line(frame, col_outline, pt1, pt3);
        draw_line(frame, col_outline, pt2, pt3);
    }

}

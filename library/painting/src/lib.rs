use random::*;
use common::date::*;

use image::{Rgb, RgbImage};
use imageproc::drawing::{Point, 
    draw_filled_rect_mut, 
    draw_filled_circle_mut, 
    draw_convex_polygon_mut,
    draw_line_segment_mut
};
use imageproc::rect::Rect;

pub const WHITE : Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
pub const BLACK : Rgb<u8> = Rgb([0u8, 0u8, 0u8]);

pub struct Painting {
    pub canvas: RgbImage,
    pub randomizer: random::RandomGenerator,
    pub width: u32,
    pub height: u32,
    pub descriptor: String,
    pub file_path: String,
}

#[derive(Debug)]
pub struct ThePoint {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct ThePointF {
    pub x: f32,
    pub y: f32
}

pub struct Circle {
    pub x0: i32,
    pub y0: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub radius: i32
}

pub struct Trapezoid {
    pub p0: ThePoint,
    pub p1: ThePoint,
    pub p2: ThePoint,
    pub p3: ThePoint,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub struct Triangle {
    pub p0: ThePoint,
    pub p1: ThePoint,
    pub p2: ThePoint,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub struct TheColor {
    pub r: u8,
    pub g: u8,
    pub b:u8
}

pub trait Painter {
    fn new(w: u32, h: u32) -> Painting;
    fn initialize(&mut self, localization: String);
    fn put_a_frame(&mut self, frame_width: i32, color: Rgb<u8>);
    fn save_file(&self);
    fn save_test(&self);
    fn fill_canvas(&mut self, r: u8, g:u8, b:u8);

    fn draw_circle(&mut self, circle: &Circle);
    fn draw_trapezoid(&mut self, trapezoid: &Trapezoid);
    fn draw_point(&mut self, point: &ThePoint, color: Rgb<u8>);
    fn draw_simple_line(&mut self, start: &ThePoint, end: &ThePoint, color: &TheColor);
    fn draw_line(&mut self, start: &ThePoint, end: &ThePoint, color: &TheColor, thickness: u32);
    fn draw_triangle(&mut self, triangle: &Triangle);
}

impl Painter for Painting {
    fn new(w: u32, h: u32) -> Painting {
        Painting {
            width: w,
            height: h,
            canvas: RgbImage::new(w, h),
            randomizer: random::RandomGenerator::new(),
            descriptor: "".to_string(),
            file_path: "".to_string()
        }
    }

    fn initialize(&mut self, localization: String) {
        self.file_path = localization;

        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(0, 0).of_size(
                    self.width as u32, 
                    self.height as u32
                ),  
            WHITE
        );
    }

    fn put_a_frame(&mut self, frame_width: i32, color: Rgb<u8>) {
        // top bar
        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(0, 0).of_size(
                self.width as u32, 
                frame_width as u32
            ),  
            color
        );
        
        // left bar
        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(0, 0).of_size(
                frame_width as u32, 
                self.height as u32
            ),
            color
        );

        // right bar
        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(
                (self.width as i32 - frame_width) as i32 , 0
            ).of_size(
                (self.width as i32 - frame_width) as u32, 
                self.height as u32
            ),
            color
        );

        // bottom bar
        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(
                0, (self.height as i32 - frame_width) as i32
            ).of_size(
                self.width as u32, 
                self.height as u32
            ),
            color
        );
    }

    fn save_file(&self) {
        let file_name = format!("{}{}{}", self.file_path, time_stamp(), ".png");
        self.canvas.save(file_name.clone()).expect("Oh.. Can not save this printing...");
    }

    fn save_test(&self) {
        let file_name = format!("{}{}", self.file_path, "test.png");
        self.canvas.save(file_name.clone()).expect("Oh.. Can not save this printing...");
    }

    fn fill_canvas(&mut self, r: u8, g:u8, b:u8) {
        draw_filled_rect_mut(
            &mut self.canvas, 
            Rect::at(
                0, 0
            ).of_size(
                self.width as u32, 
                self.height as u32
            ),
            Rgb([r, g, b])
        );
    }

    fn draw_circle(&mut self, circle: &Circle) {
        draw_filled_circle_mut(
            &mut self.canvas, 
            (circle.x0, circle.y0), 
            circle.radius , 
            Rgb([
                circle.red, 
                circle.green, 
                circle.blue
            ])
        );
    }

    fn draw_trapezoid(&mut self, trapezoid: &Trapezoid) {
        draw_convex_polygon_mut(
            &mut self.canvas,
            &[
                Point::new(trapezoid.p0.x, trapezoid.p0.y),
                Point::new(trapezoid.p1.x, trapezoid.p1.y),
                Point::new(trapezoid.p2.x, trapezoid.p2.y),
                Point::new(trapezoid.p3.x, trapezoid.p3.y)
            ],
            Rgb([
                trapezoid.red, 
                trapezoid.green, 
                trapezoid.blue
            ])
        );
    }

    fn draw_point(&mut self, point: &ThePoint, color: Rgb<u8>) {
        draw_filled_circle_mut(
            &mut self.canvas,
            (point.x, point.y),
            5, color
        );
    }

    fn draw_simple_line(&mut self, start: &ThePoint, end: &ThePoint, color: &TheColor) {
        draw_line_segment_mut(
            &mut self.canvas, 
            (start.x as f32, start.y as f32), 
            (end.x as f32, end.y as f32), 
            Rgb([color.r.clone(), color.g.clone(), color.b.clone()])
        );
    }

    fn draw_line(&mut self, start: &ThePoint, end: &ThePoint, color: &TheColor, thickness: u32) {
        draw_convex_polygon_mut(
            &mut self.canvas, 
            &[
                Point::new(start.x, start.y -thickness as i32),
                Point::new(start.x, start.y +thickness as i32),
                Point::new(end.x, end.y +thickness as i32),
                Point::new(end.x, end.y -thickness as i32)
            ],            
            Rgb([color.r.clone(), color.g.clone(), color.b.clone()])
        );
    }

    fn draw_triangle(&mut self, triangle: &Triangle) {
        draw_convex_polygon_mut(
            &mut self.canvas,
            &[
                Point::new(triangle.p0.x, triangle.p0.y),
                Point::new(triangle.p1.x, triangle.p1.y),
                Point::new(triangle.p2.x, triangle.p2.y),
            ],
            Rgb([
                triangle.red, 
                triangle.green, 
                triangle.blue
            ])
        );
    }
}
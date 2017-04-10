#![feature(alloc,collections)]
//#![feature(use_extern_macros)]
#![no_std]
//extern crate core;
extern crate collections;
use core::cmp::{max, min};
use core::f32::NAN;
use collections::vec::Vec;

fn minf(a: f32, b: f32) -> f32 {
    assert!(a != NAN && b != NAN);
    if a < b { a } else { b }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel16 {
    pub x: u16,
    pub y: u16,
}

/// Represents a 2D AABB.
#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Pixel16,
    pub max: Pixel16,
}

impl AABB {
    pub fn insert(&mut self, thing: Pixel16) -> () {
        //! This extends the AABB to fit in the given data point `thing`.
        //! This will not store the given `thing`, but only extend the size
        //! of this AABB in case it does not already fit in.
        self.max = Pixel16 {
            x: max(thing.x, self.max.x),
            y: max(thing.y, self.max.y),
        };
        self.min = Pixel16 {
            x: min(thing.x, self.max.x),
            y: min(thing.y, self.max.y),
        };
    }

    pub fn fit_in_aabb(&mut self, data: &[Pixel16]) -> Vec<Pixel16> {
        let mut scaled: Vec<Pixel16> = Vec::new();
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;
        for pos in data {
            // search extrema
            max_x = max(max_x, pos.x);
            max_y = max(max_y, pos.y);
            min_x = min(min_x, pos.x);
            min_y = min(min_y, pos.y);
        }
        // calc scale factor
        let x_factor: f32 = dx as f32 / (max_x - min_x) as f32;
        let y_factor: f32 = dy as f32 / (max_y - min_y) as f32;
        let factor = minf(x_factor, y_factor);
        // calc offset
        let x_offset: i32 = self.min.x as i32 - min_x as i32;
        let y_offset: i32 = self.min.y as i32 - min_y as i32;
        for pos in data {
            // scale points
            let mut p = Pixel16 {
                x: (pos.x.clone() as f32 * factor) as u16,
                y: (pos.y.clone() as f32 * factor) as u16,
            };
            // move points
            p.x = (p.x as i32 + x_offset) as u16;
            p.y = (p.y as i32 + y_offset) as u16;
            scaled.push(p);
        }
        scaled
    }
}

pub struct Renderer<F: FnMut(u16, u16, u32)> {
    pub pixel_col_fn: F,
}

impl<F: FnMut(u16, u16, u32)> Renderer<F> {
    pub fn draw_lines(&mut self, aabb: AABB, data: Vec<Pixel16>) -> () {
        for i in 0..data.len() - 1 {
            self.draw_line(aabb, data[i], data[i + 1], 0xFAFAFAFA);
        }
    }

    //pub fn draw_rectangles(&self, aabb: AABB, data: Vec<Pixel16>, line_thickness: u16) -> () {
    //    /// Draws a rectangle on screen.
    //    unimplemented!();
    //}

    fn draw_line(&mut self, aabb: AABB, a: Pixel16, b: Pixel16, color: u32) {
        let dx: u16;
        let dy: u16;
        if b.x >= a.x {
            dx = b.x - a.x;
        } else {
            dx = a.x - b.x;
        }
        if b.y >= a.y {
            dy = b.y - a.y;
        } else {
            dy = a.y - b.y;
        }
        let gradient: f32 = dy as f32 / dx as f32;
        let line_func = |x: f32| gradient * x + a.y as f32;
        if gradient <= 1.0 && gradient >= -1.0 {
            for x in a.x..a.x + dx + 1 {
                let y: f32 = line_func(x as f32);
                let y_decimal: f32 = y - a.y as f32;
                let intensity_low = 1.0 - y_decimal;
                let intensity_high = y_decimal;

                let pixa = (x, (y - 1.0) as u16, (intensity_low * color as f32) as u32);
                let pixb = (x, y as u16, (intensity_high * color as f32) as u32);
                if true ||
                   pixa.0 >= aabb.min.x && pixa.0 <= aabb.max.x && pixa.1 >= aabb.min.y &&
                   pixa.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixa.0, pixa.1, pixa.2);
                }
                if true ||
                   pixb.0 >= aabb.min.x && pixb.0 <= aabb.max.x && pixb.1 >= aabb.min.y &&
                   pixb.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixb.0, pixb.1, pixb.2);
                }
                //(self.pixel_col_fn)(x, (y - 1.0) as u16, (intensity_low * color as f32) as u32);
                //(self.pixel_col_fn)(x, y as u16, (intensity_high * color as f32) as u32);
            }
        } else {
            // basically, turn around axis
            for y in a.y..dy + 1 {
                let x: f32 = line_func(y as f32);
                let x_decimal: f32 = x - a.x as f32;
                let intensity_low = 1.0 - x_decimal;
                let intensity_high = x_decimal;
                let pixa = ((x - 1.0) as u16, y, (intensity_low * color as f32) as u32);
                let pixb = (x as u16, y, (intensity_high * color as f32) as u32);
                if pixa.0 >= aabb.min.x && pixa.0 <= aabb.max.x && pixa.1 >= aabb.min.y &&
                   pixa.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixa.0, pixa.1, pixa.2);
                }
                if pixb.0 >= aabb.min.x && pixb.0 <= aabb.max.x && pixb.1 >= aabb.min.y &&
                   pixb.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixb.0, pixb.1, pixb.2);
                }
            }
        }
    }
}

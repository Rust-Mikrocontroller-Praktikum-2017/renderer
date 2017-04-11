#![feature(alloc,collections)]
//#![feature(use_extern_macros)]
#![no_std]
//extern crate core;
extern crate collections;
pub mod coordinates;
use core::cmp::{max, min};
use core::f32::NAN;
use collections::vec::Vec;
use coordinates::{Coord2D, Pixel16};

fn minf(a: f32, b: f32) -> f32 {
    assert!(a != NAN && b != NAN);
    if a < b { a } else { b }
}

fn maxf(a: f32, b: f32) -> f32 {
    assert!(a != NAN && b != NAN);
    if a > b { a } else { b }
}

/// Represents a 2D AABB.
/// It will store nothing but the minimum and maximum of all "inserted" data points.
/// You can `insert` discrete Pixel coordinates, which can make the aabb bigger.
/// Or use `fit_in_aabb` to map continuous `Coord2D` into the aabb,
/// which scales and moves your data points.
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

    pub fn fit_in_aabb(&mut self, data: &[Coord2D]) -> Vec<Coord2D> {
        let mut scaled: Vec<Coord2D> = Vec::new();
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;
        let mut min_x: f32 = 0.0;
        let mut min_y: f32 = 0.0;
        for pos in data {
            // search extrema
            max_x = maxf(max_x, pos.x);
            max_y = maxf(max_y, pos.y);
            min_x = minf(min_x, pos.x);
            min_y = minf(min_y, pos.y);
        }
        // calc scale factor
        let x_factor: f32 = dx as f32 / (max_x - min_x);
        let y_factor: f32 = dy as f32 / (max_y - min_y);
        let factor = minf(x_factor, y_factor);
        // calc offset
        let x_offset: f32 = self.min.x as f32 - min_x;
        let y_offset: f32 = self.min.y as f32 - min_y;
        for pos in data {
            // scale points
            let mut p = Coord2D {
                x: pos.x * factor,
                y: pos.y * factor,
            };
            // move points
            p.x = p.x + x_offset;
            p.y = p.y + y_offset;
            scaled.push(p);
        }
        scaled
    }
}

pub struct Renderer<F: FnMut(u16, u16, u32)> {
    pub pixel_col_fn: F,
}

impl<F: FnMut(u16, u16, u32)> Renderer<F> {
    pub fn draw_lines(&mut self, aabb: AABB, data: Vec<Coord2D>) -> () {
        for i in 0..data.len() - 1 {
            self.draw_line(aabb, data[i], data[i + 1], 0xFAFAFAFA);
        }
    }

    fn draw_line(&mut self, aabb: AABB, p1: Coord2D, p2: Coord2D, color: u32) {
        let dx: f32;
        let dy: f32;
        let left;
        let right;
        if p1.x == minf(p1.x, p2.x) {
            left = p1;
            right = p2;
        } else {
            left = p2;
            right = p1;
        }
        dx = right.x - left.x;
        if right.y >= left.y {
            dy = right.y - left.y;
        } else {
            dy = left.y - right.y;
        }
        let gradient: f32 = dy / dx;
        if gradient <= 1.0 && gradient >= -1.0 {
            let line_func = |x: f32| gradient * x + left.y;
            for x in aabb.min.x..aabb.max.x + 1 {
                let y: f32 = line_func(x as f32);
                let y_decimal: f32 = y - left.y;
                let intensity_low = 1.0 - y_decimal;
                let intensity_high = y_decimal;

                let pixa = (x, (y - 1.0) as u16, (intensity_low * color as f32) as u32);
                let pixb = (x, y as u16, (intensity_high * color as f32) as u32);
                //if true ||
                //   pixa.0 >= aabb.min.x && pixa.0 <= aabb.max.x && pixa.1 >= aabb.min.y &&
                //   pixa.1 <= aabb.max.y {
                (self.pixel_col_fn)(pixa.0, pixa.1, pixa.2);
                //}
                //if true ||
                //   pixb.0 >= aabb.min.x && pixb.0 <= aabb.max.x && pixb.1 >= aabb.min.y &&
                //   pixb.1 <= aabb.max.y {
                (self.pixel_col_fn)(pixb.0, pixb.1, pixb.2);
                //}
                //(self.pixel_col_fn)(x, (y - 1.0) as u16, (intensity_low * color as f32) as u32);
                //(self.pixel_col_fn)(x, y as u16, (intensity_high * color as f32) as u32);
            }
        } else {
            // basically, turn around axis
            let line_func = |y: f32| (y - left.y) / gradient;
            for y in aabb.min.y..aabb.max.y + 1 {
                let x: f32 = line_func(y as f32);
                let x_decimal: f32 = x - left.x as f32;
                let intensity_low = 1.0 - x_decimal;
                let intensity_high = x_decimal;
                let pixa = ((x - 1.0) as u16, y, (intensity_low * color as f32) as u32);
                let pixb = (x as u16, y, (intensity_high * color as f32) as u32);
                //if pixa.0 >= aabb.min.x && pixa.0 <= aabb.max.x && pixa.1 >= aabb.min.y &&
                //   pixa.1 <= aabb.max.y {
                (self.pixel_col_fn)(pixa.0, pixa.1, pixa.2);
                //}
                //if pixb.0 >= aabb.min.x && pixb.0 <= aabb.max.x && pixb.1 >= aabb.min.y &&
                //   pixb.1 <= aabb.max.y {
                (self.pixel_col_fn)(pixb.0, pixb.1, pixb.2);
                //}
            }
        }
    }
}

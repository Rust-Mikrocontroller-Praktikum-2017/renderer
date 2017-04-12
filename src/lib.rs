#![feature(alloc,collections)]
//#![feature(use_extern_macros)]
#![no_std]
//extern crate core;
#[macro_use]
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
        const EPS: f32 = 0.0001;
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
                x: maxf((pos.x * (factor - EPS)), 0.0),
                y: maxf((pos.y * (factor - EPS)), 0.0),
            };
            // move points
            p.x = p.x + x_offset + EPS;
            p.y = p.y + y_offset + EPS;
            // fits in screen?
            assert!(p.x <= 480.0);
            assert!(p.y <= 272.0);
            assert!(p.x >= 0.0);
            assert!(p.y >= 0.0);
            // fits in aabb?
            assert!(p.x <= self.max.x as f32);
            assert!(p.y <= self.max.y as f32);
            assert!(p.x >= self.min.x as f32);
            assert!(p.y >= self.min.y as f32);
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
            self.draw_line(data[i], data[i + 1], 0xFAFAFAFA);
        }
    }

    fn draw_line(&mut self, p1: Coord2D, p2: Coord2D, color: u32) {
        //println!("\nNew LINE!");
        let dx: f32;
        let dy: f32;
        let left_x; // point on the left
        let right_x;
        let left_y;
        let right_y;
        if p1.x == minf(p1.x, p2.x) {
            left_x = p1;
            right_x = p2;
        } else {
            left_x = p2;
            right_x = p1;
        }
        if p1.y == minf(p1.y, p2.y) {
            left_y = p1;
            right_y = p2;
        } else {
            left_y = p2;
            right_y = p1;
        }
        dx = right_x.x - left_x.x;
        dy = right_x.y - left_x.y;
        let aabb = AABB {
            min: Pixel16 {
                x: left_x.x as u16,
                y: maxf(minf(p1.y, p2.y), 0.0) as u16,
            },
            max: Pixel16 {
                x: right_x.x as u16,
                y: maxf(p1.y, p2.y) as u16,
            },
        };
        let gradient: f32 = dy / dx;
        if gradient <= 1.0 && gradient >= -1.0 {
            //println!("plot X");
            let left = left_x;
            let line_func = |x: f32| gradient * x + left.y;
            for x in aabb.min.x..aabb.max.x + 1 {
                let y: f32 = line_func((x - aabb.min.x) as f32);
                let y_even: i32 = y as i32;
                let mut y_decimal: f32 = y - (y_even as f32);
                if y_decimal < 0.0 {
                    y_decimal *= -1.0;
                }
                assert!(y_decimal <= 1.0 && y_decimal >= -1.0);
                let intensity_low = 1.0 - y_decimal;
                let intensity_high = y_decimal;
                //println!("y:{}", y_even);
                let pixa = (x, (y - 1.0) as u16, (intensity_low * color as f32) as u32);
                let pixb = (x, y as u16, (intensity_high * color as f32) as u32);
                //assert!(pixa.0 < 480);
                //assert!(pixb.0 < 480);
                //assert!(pixa.0 >= 0);
                //assert!(pixb.0 >= 0);
                //assert!(pixa.1 < 272);
                //assert!(pixb.1 < 272);
                //assert!(pixa.1 >= 0);
                //assert!(pixb.1 >= 0);
                if pixa.0 >= aabb.min.x && pixa.0 <= aabb.max.x && pixa.1 >= aabb.min.y &&
                   pixa.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixa.0, pixa.1, pixa.2);
                }
                if pixb.0 >= aabb.min.x && pixb.0 <= aabb.max.x && pixb.1 >= aabb.min.y &&
                   pixb.1 <= aabb.max.y {
                    (self.pixel_col_fn)(pixb.0, pixb.1, pixb.2);
                }
            }
        } else {
            //println!("plot Y");
            // basically, turn around axis
            let left = left_y;
            let line_func = |y: f32| (y - left.y) / gradient;
            for y in aabb.min.y..aabb.max.y + 1 {
                let x: f32 = line_func((y - aabb.min.y) as f32); // + aabb.min.x as f32;
                let x_even: i32 = x as i32;
                let mut x_decimal: f32 = x - (x_even as f32);
                if x_decimal < 0.0 {
                    x_decimal *= -1.0;
                }
                assert!(x_decimal <= 1.0 && x_decimal >= -1.0);
                let intensity_low = 1.0 - x_decimal;
                let intensity_high = x_decimal;
                let pixa = ((x - 1.0) as u16, y, (intensity_low * color as f32) as u32);
                let pixb = (x as u16, y, (intensity_high * color as f32) as u32);
                //assert!(pixa.0 < 480);
                //assert!(pixb.0 < 480);
                //assert!(pixa.0 >= 0);
                //assert!(pixb.0 >= 0);
                //assert!(pixa.1 < 272);
                //assert!(pixb.1 < 272);
                //assert!(pixa.1 >= 0);
                //assert!(pixb.1 >= 0);
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


#[test]
fn test_draw_line() {
    use collections::vec;
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 100, y: 100 },
    };
    let data = vec![Coord2D { x: 0f32, y: 0f32 },
                    Coord2D { x: 0f32, y: 1f32 },
                    Coord2D { x: 1f32, y: 1f32 },
                    Coord2D { x: 1f32, y: 0f32 }];
    let mut result: Vec<Pixel16> = Vec::new();
    {
        let func = |x, y, color| result.push(Pixel16 { x: x, y: y });
        let mut r = Renderer { pixel_col_fn: func };

        let to_plot = aabb.fit_in_aabb(&data);
        r.draw_lines(aabb, to_plot);
    }
    for pix in result {
        assert!(pix.x < 480);
        assert!(pix.y < 272);
    }
}

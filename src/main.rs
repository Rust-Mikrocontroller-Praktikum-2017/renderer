extern crate renderer;
use renderer::{Pixel16, AABB, Renderer};

fn main() {
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 15, y: 20 },
    };
    let p1 = Pixel16 { x: 0, y: 0 };
    let p2 = Pixel16 { x: 500, y: 500 };
    let func = |x, y, color| print!("({}, {})*{:x} ", x, y, color);
    let mut r = Renderer { pixel_col_fn: func };

    let data = aabb.fit_in_aabb(&[p1, p2]);
    println!("{:?}", [p1, p2]);
    println!("{:?}", data);
    r.draw_lines(aabb, data);
}

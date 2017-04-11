extern crate renderer;
use renderer::{AABB, Renderer};
use renderer::coordinates::{Pixel16, Coord2D};

fn main() {
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 15, y: 20 },
    };
    let p1 = Coord2D { x: 0.0, y: 0.0 };
    let p2 = Coord2D { x: 500.0, y: 700.0 };
    let func = |x, y, color| print!("({}, {}), ", x, y);
    let mut r = Renderer { pixel_col_fn: func };

    let data = aabb.fit_in_aabb(&[p1, p2]);
    println!("{:?}", [p1, p2]);
    println!("{:?}", data);
    r.draw_lines(aabb, data);
}

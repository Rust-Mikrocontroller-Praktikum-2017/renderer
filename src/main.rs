extern crate renderer;
use renderer::{AABB, Renderer};
use renderer::coordinates::{Pixel16, Coord2D};

fn main() {
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 100, y: 100 },
    };
    let data = vec![Coord2D { x: 0f32, y: 0f32 },
                    Coord2D { x: 0f32, y: 1f32 },
                    Coord2D { x: 1f32, y: 1f32 },
                    Coord2D { x: 1f32, y: 0f32 }];
    let func = |x, y, color| print!("({}, {}), ", x, y);
    let mut r = Renderer { pixel_col_fn: func };

    let to_plot = aabb.fit_in_aabb(&data);
    println!("points:  {:?}", data);
    println!("in aabb: {:?}", to_plot);
    r.draw_lines(aabb, to_plot);
}

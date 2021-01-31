//| sierpinski triangle image generator
extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


/// Points use to build the triangle and plot point on the canvas
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: u32,
    y: u32
}

/// main function
fn main() {
    let origin = Point {
        x: 0,
        y: 0
    };

    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let cnt: u32 = 5_000_000;

    let pts: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0},
        Point { x: 0, y: HEIGHT},
        Point { x: WIDTH, y: HEIGHT},
    ];

    let mut p = Point {
        x: 350,
        y: 350
    };

    let mut num: usize;
    let pixel = img[(0, 0)];

    for _ in 0..cnt {
        num = rand::thread_rng().gen_range(0..3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout = File::create(&Path::new("triangle.png")).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);

    println!("The origin point is at: {:?}", origin);
}

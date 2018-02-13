extern crate cgmath;
extern crate image;
extern crate rand;

pub mod perlin;
pub mod grid;
pub mod interpolate;

use std::path::Path;
use std::io;
use std::f64;

fn main() {
    let perlin = perlin::Perlin::new(
        (30, 30),
        &mut perlin::RandomGradientBuilder2d::new(rand::thread_rng()),
        interpolate::ImprovedPerlinInterpolator::new(),
    );

    save_perlin_image(&perlin, (600, 600), Path::new("out.png")).unwrap();
}

fn save_perlin_image<P>(
    perlin: &perlin::Perlin<P>,
    dimensions: (u32, u32),
    path: &Path,
) -> io::Result<()>
where
    P: interpolate::InterpolationFunction,
{
    let (img_width, img_height) = dimensions;
    let mut img = image::ImageBuffer::new(img_width, img_height);

    let dx = f64::from(perlin.width()) / f64::from(img_width);
    let dy = f64::from(perlin.height()) / f64::from(img_height);

    let (mut min, mut max) = (f64::MAX, f64::MIN);

    for y in 0..img_height {
        let perlin_y = f64::from(y) * dy;
        for x in 0..img_width {
            let perlin_x = f64::from(x) * dx;

            let value = 0.5 + perlin.get_value(cgmath::Vector2::new(perlin_x, perlin_y));

            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }

            img[(x, y)] = image::Luma([(value * 255.0) as u8]);
        }
    }

    println!("{} {}", min, max);

    img.save(path)
}

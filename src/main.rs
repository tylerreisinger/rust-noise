extern crate cgmath;
extern crate image;
extern crate rand;

pub mod perlin;
pub mod grid;
pub mod interpolate;
pub mod octave;
pub mod noise;

use std::path::Path;
use std::io;
use std::f64;

use noise::Noise;

fn main() {
    let perlin = perlin::Perlin::new(
        (4, 4),
        &mut perlin::RandomGradientBuilder2d::new(rand::thread_rng()),
        interpolate::ImprovedPerlinInterpolator::new(),
    );
    let perlin_2 = perlin::Perlin::new(
        (8, 8),
        &mut perlin::RandomGradientBuilder2d::new(rand::thread_rng()),
        interpolate::ImprovedPerlinInterpolator::new(),
    );

    let octaves = octave::OctaveNoise::from_octaves(vec![
        octave::Octave::new(perlin.clone(), 0.667),
        octave::Octave::new(perlin_2.clone(), 0.333),
    ]);

    let octaves_2 = perlin::build_geometric_octaves(
        (2, 2),
        8,
        2.0,
        &mut perlin::RandomGradientBuilder2d::new(rand::thread_rng()),
        &interpolate::ImprovedPerlinInterpolator::new(),
    );

    let octaves_3 = perlin::build_arithmetic_octaves(
        (4, 4),
        50,
        8.0,
        2,
        &mut perlin::RandomGradientBuilder2d::new(rand::thread_rng()),
        &interpolate::ImprovedPerlinInterpolator::new(),
    );

    save_noise_image(&octaves_2, (800, 800), Path::new("out.png")).unwrap();
    println!("{}", octaves_2);
}

fn save_noise_image<N: Noise>(perlin: &N, dimensions: (u32, u32), path: &Path) -> io::Result<()> {
    let (img_width, img_height) = dimensions;
    let mut img = image::ImageBuffer::new(img_width, img_height);

    let dx = 1.0 / f64::from(img_width);
    let dy = 1.0 / f64::from(img_height);

    let (mut min, mut max) = (f64::MAX, f64::MIN);

    for y in 0..img_height {
        let perlin_y = f64::from(y) * dy;
        for x in 0..img_width {
            let perlin_x = f64::from(x) * dx;

            let value = 2.5 * perlin.value_at(cgmath::Vector2::new(perlin_x, perlin_y));

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

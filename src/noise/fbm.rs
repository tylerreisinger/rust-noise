use noise::{Noise, OctaveNoise, Perlin1d, Perlin2d, Perlin3d, Point1, Point2, Point3, ToTuple,
            WithFrequency};
use noise::octave::build_geometric_fractal_noise;
use interpolate::{self, InterpolationFunction};
use gradient::{PermutedGradientTable, RandomGradientBuilder1d, RandomGradientBuilder2d,
               RandomGradientBuilder3d};

use rand::Rng;
use cgmath::{Vector2, Vector3};

pub type DefaultInterpolator = interpolate::Hermite5thOrderInterpolator;

macro_rules! impl_fbm {
    ($name:ident, $freq:ty, $dim:ty, $builder:ident,
     $noise:ident, $vector:ty, $point:ty, $default_freq:expr, $default_scale:expr) => (

        #[derive(Clone, Debug)]
        pub struct $name<P>
        where
            P: InterpolationFunction,
        {
            frequency: $freq,
            frequency_scaling: $freq,
            persistance: f64,
            interp: P,
            noise_octaves: OctaveNoise<$noise<PermutedGradientTable<$vector>, P>>,
        }

        impl $name<DefaultInterpolator> {
            pub fn new<R: Rng + Clone>(rng: &mut R) -> $name<DefaultInterpolator> {
                let mut fbm = $name {
                    frequency: Self::DEFAULT_FREQUENCY,
                    frequency_scaling: Self::DEFAULT_SCALING,
                    persistance: Self::DEFAULT_PERSISTANCE,
                    interp: DefaultInterpolator::new(),
                    noise_octaves: OctaveNoise::new(),
                };
                fbm.build_noise(rng, Self::DEFAULT_NUM_OCTAVES);
                fbm
            }
        }

        impl<P> WithFrequency for $name<P>
        where P: InterpolationFunction + Clone,
        {
            fn with_frequency(self, frequency: $dim) -> Self {
                $name {
                    frequency: frequency.unwrap(),
                    noise_octaves: self.noise_octaves.with_frequency(frequency.clone()),
                    ..self
                }
            }
        }

        impl<P> $name<P>
        where P: InterpolationFunction + Clone,
        {
            pub const DEFAULT_FREQUENCY: $freq = $default_freq;
            pub const DEFAULT_NUM_OCTAVES: usize = 8;
            pub const DEFAULT_SCALING: $freq = $default_scale;
            pub const DEFAULT_PERSISTANCE: f64 = 2.0;

            pub fn with_frequency_scaling(self, frequency_scaling: $freq) -> Self {
                $name {
                    frequency_scaling,
                    ..self
                }
            }
            pub fn with_persistance(self, persistance: f64) -> Self {
                $name {
                    persistance,
                    ..self
                }
            }
            pub fn with_num_octaves<R: Rng + Clone>(mut self, rng: &mut R, num_octaves: usize) -> Self {
                self.build_noise(rng, num_octaves);
                self
            }

            pub fn frequency(&self) -> $freq {
                self.frequency
            }
            pub fn frequency_scaling(&self) -> $freq {
                self.frequency_scaling
            }
            pub fn persistance(&self) -> f64 {
                self.persistance
            }
            pub fn num_octaves(&self) -> usize {
                self.noise_octaves.num_octaves()
            }

            fn make_default_gradient_provider<R: Rng + Clone>(rng: &mut R, size: u32) -> PermutedGradientTable<$vector> {
                let mut builder = $builder::new(rng.clone());
                PermutedGradientTable::new(rng, &mut builder, size)
            }


        }
        impl<P> $name<P>
        where P: InterpolationFunction + Clone,
        {
            fn build_noise<R: Rng + Clone>(&mut self, rng: &mut R, num_octaves: usize) {
                let octaves = build_geometric_fractal_noise(
                    self.frequency.to_tuple(),
                    num_octaves as u32,
                    self.frequency_scaling.to_tuple(),
                    self.persistance,
                    &mut |_, freq, _| {
                        let f: $dim = freq;
                        $noise::new(f.unwrap(),
                            Self::make_default_gradient_provider(rng, 256)).with_interpolator(self.interp.clone())
                    }
                );
                self.noise_octaves = octaves;
            }
        }

        impl<P> Noise for $name<P>
        where P: InterpolationFunction
        {
            type IndexType = $point;
            type DimType = $dim;

            fn value_at(&self, pos: Self::IndexType) -> f64 {
                self.noise_octaves.value_at(pos)
            }
            fn frequency(&self) -> Self::DimType {
                self.noise_octaves.frequency()
            }
        }
    )
}

impl_fbm!(
    Fbm1d,
    f64,
    (f64,),
    RandomGradientBuilder1d,
    Perlin1d,
    f64,
    Point1<f64>,
    1.0,
    2.0
);
impl_fbm!(
    Fbm2d,
    (f64, f64),
    (f64, f64),
    RandomGradientBuilder2d,
    Perlin2d,
    Vector2<f64>,
    Point2<f64>,
    (1.0, 1.0),
    (2.0, 2.0)
);
impl_fbm!(
    Fbm3d,
    (f64, f64, f64),
    (f64, f64, f64),
    RandomGradientBuilder3d,
    Perlin3d,
    Vector3<f64>,
    Point3<f64>,
    (1.0, 1.0, 1.0),
    (2.0, 2.0, 2.0)
);

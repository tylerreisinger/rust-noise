use std::mem;

use noise::{Noise, Perlin1d, Perlin2d, Perlin3d, Point1, Point2, Point3, TupleUtil, WithFrequency};
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
        pub struct $name<P, R>
        where
            P: InterpolationFunction,
            R: Rng + Clone
        {
            frequency: $freq,
            frequency_scaling: $freq,
            persistance: f64,
            interp: P,
            octaves: Vec<$noise<PermutedGradientTable<$vector>, P>>,
            rng: R,
        }

        impl<R: Rng + Clone> $name<DefaultInterpolator, R> {
            pub fn new(rng: R)-> $name<DefaultInterpolator, R> {
                let mut fbm = $name {
                    frequency: Self::DEFAULT_FREQUENCY,
                    frequency_scaling: Self::DEFAULT_SCALING,
                    persistance: Self::DEFAULT_PERSISTANCE,
                    interp: DefaultInterpolator::default(),
                    octaves: Vec::new(),
                    rng: rng,
                };
                fbm.build_noise(Self::DEFAULT_NUM_OCTAVES);
                fbm
            }
        }

        impl<P, R> $name<P, R>
        where P: InterpolationFunction + Clone,
              R: Rng + Clone,
        {
            pub const DEFAULT_FREQUENCY: $freq = $default_freq;
            pub const DEFAULT_NUM_OCTAVES: usize = 8;
            pub const DEFAULT_SCALING: $freq = $default_scale;
            pub const DEFAULT_PERSISTANCE: f64 = 2.0;

            pub fn with_interpolator<P2>(self, interp: P2) -> $name<P2, R>
                where P2: InterpolationFunction + Clone
            {
                $name {
                    frequency: self.frequency,
                    frequency_scaling: self.frequency_scaling,
                    persistance: self.persistance,
                    interp: interp.clone(),
                    octaves: self.octaves.into_iter()
                        .map(|x| x.with_interpolator(interp.clone())).collect(),
                    rng: self.rng,
                }
            }

            pub fn with_frequency(self, frequency: $dim) -> Self {
                let mut new = $name {
                    frequency: frequency,
                    ..self
                };
                new.set_new_noise_frequencies();
                new
            }
            pub fn with_frequency_scaling(self, frequency_scaling: $dim) -> Self {
                let mut new = $name {
                    frequency_scaling: frequency_scaling,
                    ..self
                };
                new.set_new_noise_frequencies();
                new
            }

            pub fn with_num_octaves(self, num_octaves: usize) -> Self {
                let mut new = $name {
                    ..self
                };
                new.build_noise(num_octaves);
                new
            }

            pub fn with_persistance(self, persistance: f64) -> Self {
                $name {
                    persistance,
                    ..self
                }
            }
            pub fn frequency_scaling(&self) -> $freq {
                self.frequency_scaling
            }
            pub fn persistance(&self) -> f64 {
                self.persistance
            }
            pub fn num_octaves(&self) -> usize {
                self.octaves.len()
            }
            pub fn octaves(&self) -> &Vec<$noise<PermutedGradientTable<$vector>, P>> {
                &self.octaves
            }

            fn make_default_gradient_provider(&mut self, size: u32)
                -> PermutedGradientTable<$vector>
            {
                let mut builder = $builder::new(self.rng.clone());
                PermutedGradientTable::new(&mut self.rng, &mut builder, size)
            }

            fn build_noise(&mut self, num_octaves: usize) {
                let mut octaves = Vec::with_capacity(num_octaves);

                let mut frequency = self.frequency();
                for _ in 0..num_octaves {
                    let octave = $noise::new(frequency, self.make_default_gradient_provider(256))
                        .with_interpolator(self.interp.clone());
                    octaves.push(octave);

                    frequency = frequency.apply(self.frequency_scaling().clone(), |f, s| f * s);
                }

                self.octaves = octaves;
            }

            fn set_new_noise_frequencies(&mut self) {
                let new_octaves = Vec::with_capacity(self.num_octaves());
                let octaves = mem::replace(&mut self.octaves, new_octaves);
                let mut frequency = self.frequency();

                for o in octaves.into_iter() {
                    self.octaves.push(o.with_frequency(frequency));
                    frequency = frequency.apply(
                        self.frequency_scaling().clone(), |f, s| f * s);
                }
            }
        }
        impl<P, R> Noise for $name<P, R>
        where P: InterpolationFunction + Clone,
              R: Rng + Clone,
        {
            type IndexType = $point;
            type DimType = $dim;

            fn value_at(&self, pos: Self::IndexType) -> f64 {
                let amplitude_multiplier: f64 = 1.0
                    / (0..self.num_octaves())
                        .map(|x| 1.0 / (self.persistance.powi(x as i32)))
                        .sum::<f64>();

                let mut amplitude = amplitude_multiplier;
                let mut val = 0.0;
                for o in &self.octaves {
                    let octave_val = o.value_at(pos);
                    let scaled_val = octave_val * amplitude;

                    val += scaled_val;

                    amplitude /= self.persistance;
                }

                val
            }
            fn frequency(&self) -> Self::DimType {
                self.frequency
            }
        }
    );
}

impl_fbm!(
    Fbm1d,
    f64,
    f64,
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

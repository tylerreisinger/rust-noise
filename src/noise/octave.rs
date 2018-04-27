use std::fmt;

use noise::{Noise, TupleUtil, WithFrequency};

#[derive(Clone, Debug)]
pub struct Octave<T: Noise> {
    noise: T,
    amplitude: f64,
}

#[derive(Clone, Debug, Default)]
pub struct OctaveNoise<T: Noise> {
    octaves: Vec<Octave<T>>,
}

impl<T> Octave<T>
where
    T: Noise,
{
    pub fn new(noise: T, amplitude: f64) -> Octave<T> {
        Octave { noise, amplitude }
    }

    pub fn noise(&self) -> &T {
        &self.noise
    }
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn inner_noise(&self) -> &T {
        &self.noise
    }
    pub fn inner_noise_mut(&mut self) -> &mut T {
        &mut self.noise
    }
}

impl<T> WithFrequency for Octave<T>
where
    T: Noise + WithFrequency,
{
    fn with_frequency(self, frequency: Self::DimType) -> Self {
        Self {
            noise: self.noise.with_frequency(frequency),
            ..self
        }
    }
}

impl<T> OctaveNoise<T>
where
    T: Noise,
{
    pub fn new() -> OctaveNoise<T> {
        OctaveNoise {
            octaves: Vec::new(),
        }
    }

    pub fn from_octaves(octaves: Vec<Octave<T>>) -> OctaveNoise<T> {
        OctaveNoise { octaves }
    }

    pub fn num_octaves(&self) -> usize {
        self.octaves.len()
    }
}

impl<T> Noise for Octave<T>
where
    T: Noise,
{
    type IndexType = T::IndexType;
    type DimType = T::DimType;

    fn value_at(&self, pos: T::IndexType) -> f64 {
        self.noise.value_at(pos) * self.amplitude
    }

    fn frequency(&self) -> T::DimType {
        self.noise.frequency()
    }
}

impl<T> WithFrequency for OctaveNoise<T>
where
    T: Noise + WithFrequency,
    T::DimType: Default + Clone,
{
    fn with_frequency(self, frequency: Self::DimType) -> Self {
        Self {
            octaves: self.octaves
                .into_iter()
                .map(|x| x.with_frequency(frequency.clone()))
                .collect(),
        }
    }
}

impl<T> Noise for OctaveNoise<T>
where
    T: Noise,
    T::DimType: Default,
{
    type IndexType = T::IndexType;
    type DimType = T::DimType;

    fn value_at(&self, pos: T::IndexType) -> f64 {
        self.octaves
            .iter()
            .fold(0.0, |l, o| l + o.value_at(pos.clone()))
    }

    fn frequency(&self) -> T::DimType {
        if self.octaves.is_empty() {
            self.octaves[0].frequency()
        } else {
            Default::default()
        }
    }
}

impl<T> fmt::Display for Octave<T>
where
    T: Noise,
    T::DimType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Octave <{:?}, A={}>", self.frequency(), self.amplitude())
    }
}

impl<T> fmt::Display for OctaveNoise<T>
where
    T: Noise,
    T::DimType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "OctaveNoise [")?;
        for octave in &self.octaves {
            writeln!(f, "\t{},", octave)?;
        }
        writeln!(f, "]")
    }
}

pub fn build_geometric_fractal_noise<N, F>(
    initial_frequency: N::DimType,
    num_octaves: u32,
    frequency_scaling: N::DimType,
    persistance: f64,
    noise_builder: &mut F,
) -> OctaveNoise<N>
where
    N: Noise,
    N::DimType: TupleUtil<f64> + Clone + fmt::Debug,
    F: FnMut(u32, N::DimType, f64) -> N,
{
    let mut octaves = Vec::with_capacity(num_octaves as usize);
    let amplitude_multiplier: f64 = 1.0
        / (0..num_octaves)
            .map(|x| 1.0 / (persistance.powi(x as i32 + 1)))
            .sum::<f64>();

    let scaling = frequency_scaling;
    let mut frequency = initial_frequency;
    for i in 0..num_octaves {
        let amplitude = (1.0 / persistance.powi(i as i32 + 1)) * amplitude_multiplier;
        let octave = Octave::new(
            noise_builder(i as u32, frequency.clone(), amplitude),
            amplitude,
        );
        frequency = frequency.apply(scaling.clone(), |f, s| f * s);
        octaves.push(octave);
    }

    OctaveNoise::from_octaves(octaves)
}

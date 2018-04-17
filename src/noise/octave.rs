use std::fmt;

use noise::Noise;

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

    fn dimensions(&self) -> T::DimType {
        self.noise.dimensions()
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

    fn dimensions(&self) -> T::DimType {
        if self.octaves.is_empty() {
            self.octaves[0].dimensions()
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
        write!(
            f,
            "Octave <{:?}, A={}>",
            self.dimensions(),
            self.amplitude()
        )
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

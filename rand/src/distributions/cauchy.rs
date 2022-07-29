use crate::distributions::{Distribution, StandardUniformOpenOpen};
use crate::rngs::Rng;

pub struct StandardCauchy {}

pub struct Cauchy {
    location: f64,
    scale: f64,
}

impl Cauchy {
    pub fn new(location: f64, scale: f64) -> Self {
        Self { location, scale }
    }
}

impl Distribution<f64> for StandardCauchy {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let p = rng.sample(&StandardUniformOpenOpen {});

        f64::tan(core::f64::consts::PI * (p - 0.5))
    }
}

impl Distribution<f64> for Cauchy {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let c = rng.sample(&StandardCauchy {});

        f64::mul_add(c, self.scale, self.location)
    }
}

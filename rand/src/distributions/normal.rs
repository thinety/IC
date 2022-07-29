use crate::distributions::{Distribution, StandardUniformClosedOpen, StandardUniformOpenClosed};
use crate::rngs::Rng;

pub struct StandardNormal {}

pub struct Normal {
    mean: f64,
    std_dev: f64,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self { mean, std_dev }
    }
}

impl Distribution<(f64, f64)> for StandardNormal {
    // Box-Muller transform
    fn sample<R>(&self, rng: &mut R) -> (f64, f64)
    where
        R: Rng,
    {
        let u1 = rng.sample(&StandardUniformOpenClosed {});
        let u2 = rng.sample(&StandardUniformClosedOpen {});

        let r = f64::sqrt(-2.0 * f64::ln(u1));
        let (sint, cost) = f64::sin_cos(2.0 * core::f64::consts::PI * u2);

        (r * cost, r * sint)
    }
}

impl Distribution<(f64, f64)> for Normal {
    fn sample<R>(&self, rng: &mut R) -> (f64, f64)
    where
        R: Rng,
    {
        let (z1, z2) = rng.sample(&StandardNormal {});

        (
            f64::mul_add(z1, self.std_dev, self.mean),
            f64::mul_add(z2, self.std_dev, self.mean),
        )
    }
}

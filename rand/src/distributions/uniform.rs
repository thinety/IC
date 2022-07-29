use crate::distributions::Distribution;
use crate::rngs::Rng;

pub struct Uniform {
    start: u64,
    range: u64,
}

impl Uniform {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl Distribution<u64> for Uniform {
    // https://arxiv.org/pdf/1805.10941.pdf
    fn sample<R>(&self, rng: &mut R) -> u64
    where
        R: Rng,
    {
        let x = rng.gen();
        let mut m = (x as u128) * (self.range as u128);
        let mut l = m as u64;

        if l < self.range {
            let t = (u64::MAX - self.range + 1) % self.range;

            while l < t {
                let x = rng.gen();
                m = (x as u128) * (self.range as u128);
                l = m as u64;
            }
        }

        self.start + ((m >> 64) as u64)
    }
}

pub struct StandardUniformClosedOpen {}

pub struct UniformClosedOpen {
    start: f64,
    range: f64,
}

pub struct StandardUniformOpenClosed {}

pub struct UniformOpenClosed {
    start: f64,
    range: f64,
}

pub struct StandardUniformOpenOpen {}

pub struct UniformOpenOpen {
    start: f64,
    range: f64,
}

impl UniformClosedOpen {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl UniformOpenClosed {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl UniformOpenOpen {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl Distribution<f64> for StandardUniformClosedOpen {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let value = rng.gen() >> 11;

        let scale = 1.0 / ((1u64 << 53) as f64);

        scale * (value as f64)
    }
}

impl Distribution<f64> for UniformClosedOpen {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let x = rng.sample(&StandardUniformClosedOpen {});

        f64::mul_add(x, self.range, self.start)
    }
}

impl Distribution<f64> for StandardUniformOpenClosed {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let value = rng.gen() >> 11;

        let scale = 1.0 / ((1u64 << 53) as f64);

        scale * ((value + 1) as f64)
    }
}

impl Distribution<f64> for UniformOpenClosed {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let x = rng.sample(&StandardUniformOpenClosed {});

        f64::mul_add(x, self.range, self.start)
    }
}

impl Distribution<f64> for StandardUniformOpenOpen {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let value = rng.gen() >> 11;

        let scale = 1.0 / ((1u64 << 53) as f64);

        scale * ((value | 1) as f64)
    }
}

impl Distribution<f64> for UniformOpenOpen {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng,
    {
        let x = rng.sample(&StandardUniformOpenOpen {});

        f64::mul_add(x, self.range, self.start)
    }
}

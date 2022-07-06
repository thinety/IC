use crate::rngs::Rng;

use super::Distribution;

pub struct UniformClosedOpen01 {}

pub struct UniformOpenClosed01 {}

pub struct UniformOpenOpen01 {}

macro unit_uniform_distributions_impl($fty:ty, $uty:ty, $total_bits:expr, $significant_bits:expr) {
    impl Distribution<$fty, $uty> for UniformClosedOpen01 {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            let value = rng.gen() >> ($total_bits - $significant_bits);

            let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

            scale * (value as $fty)
        }
    }

    impl Distribution<$fty, $uty> for UniformOpenClosed01 {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            let value = rng.gen() >> ($total_bits - $significant_bits);

            let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

            scale * ((value + 1) as $fty)
        }
    }

    impl Distribution<$fty, $uty> for UniformOpenOpen01 {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            let value = rng.gen() >> ($total_bits - $significant_bits);

            let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

            scale * ((value | 1) as $fty)
        }
    }
}

unit_uniform_distributions_impl! { f32, u32, 32, 24 }
unit_uniform_distributions_impl! { f64, u64, 64, 53 }

pub struct UniformClosedOpen<T> {
    start: T,
    end: T,
}
impl<T> UniformClosedOpen<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

pub struct UniformOpenOpen<T> {
    start: T,
    end: T,
}
impl<T> UniformOpenOpen<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

pub struct UniformOpenClosed<T> {
    start: T,
    end: T,
}
impl<T> UniformOpenClosed<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

macro uniform_distributions_impl($fty:ty, $uty:ty) {
    impl Distribution<$fty, $uty> for UniformClosedOpen<$fty> {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            rng.sample(&UniformClosedOpen01 {}) * (self.end - self.start) + self.start
        }
    }

    impl Distribution<$fty, $uty> for UniformOpenClosed<$fty> {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            rng.sample(&UniformOpenClosed01 {}) * (self.end - self.start) + self.start
        }
    }

    impl Distribution<$fty, $uty> for UniformOpenOpen<$fty> {
        fn sample<R>(&self, rng: &mut R) -> $fty
        where
            R: Rng<$uty>,
        {
            rng.sample(&UniformOpenOpen01 {}) * (self.end - self.start) + self.start
        }
    }
}

uniform_distributions_impl! { f32, u32 }
uniform_distributions_impl! { f64, u64 }

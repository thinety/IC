use crate::distributions::Distribution;
use crate::rngs::Rng;

pub struct StandardUniform {}

pub struct Uniform<T> {
    start: T,
    range: T,
}

macro_rules! uniform_impl {
    ($ty:ty) => {
        impl Uniform<$ty> {
            pub fn new(start: $ty, end: $ty) -> Self {
                Self {
                    start,
                    range: end - start,
                }
            }
        }

        impl From<core::ops::Range<$ty>> for Uniform<$ty> {
            fn from(range: core::ops::Range<$ty>) -> Self {
                Self::new(range.start, range.end)
            }
        }
    };
}

uniform_impl! { u32 }
uniform_impl! { u64 }
uniform_impl! { f32 }
uniform_impl! { f64 }

macro_rules! uniform_discrete_distribution_impl {
    ($uty:tt, $wty:ty, $shift:expr) => {
        impl Distribution<$uty> for Uniform<$uty> {
            type Backend = $uty;

            // https://arxiv.org/pdf/1805.10941.pdf
            fn sample<R>(&self, rng: &mut R) -> $uty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.gen();
                let mut m = (x as $wty) * (self.range as $wty);
                let mut l = m as $uty;

                if l < self.range {
                    let t = ($uty::MAX - self.range + 1) % self.range;

                    while l < t {
                        let x = rng.gen();
                        m = (x as $wty) * (self.range as $wty);
                        l = m as $uty;
                    }
                }

                self.start + (m >> $shift) as $uty
            }
        }
    };
}

uniform_discrete_distribution_impl! { u32, u64, 32 }
uniform_discrete_distribution_impl! { u64, u128, 64 }

macro_rules! uniform_continuous_distribution_impl {
    ($fty:tt, $uty:ty, $total_bits:expr, $significant_bits:expr) => {
        impl Distribution<$fty> for StandardUniform {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let value = rng.gen() >> ($total_bits - $significant_bits);

                let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

                scale * (value as $fty)
            }
        }

        impl Distribution<$fty> for Uniform<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.sample(&StandardUniform {});

                $fty::mul_add(x, self.range, self.start)
            }
        }
    };
}

uniform_continuous_distribution_impl! { f32, u32, 32, 24 }
uniform_continuous_distribution_impl! { f64, u64, 64, 53 }

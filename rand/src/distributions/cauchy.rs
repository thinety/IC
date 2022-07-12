use crate::distributions::{Distribution, StandardUniformOpenOpen};
use crate::rngs::Rng;

pub struct StandardCauchy {}

pub struct Cauchy<T> {
    location: T,
    scale: T,
}

macro_rules! cauchy_impl {
    ($fty:tt, $uty:ty) => {
        impl Cauchy<$fty> {
            pub fn new(location: $fty, scale: $fty) -> Self {
                Self { location, scale }
            }
        }

        impl Distribution<$fty> for StandardCauchy {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let p: $fty = rng.sample(&StandardUniformOpenOpen {});

                $fty::tan(core::$fty::consts::PI * (p - 0.5))
            }
        }

        impl Distribution<$fty> for Cauchy<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let c = rng.sample(&StandardCauchy {});

                $fty::mul_add(c, self.scale, self.location)
            }
        }
    };
}

cauchy_impl! { f32, u32 }
cauchy_impl! { f64, u64 }

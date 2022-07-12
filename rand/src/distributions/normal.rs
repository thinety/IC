use crate::distributions::{Distribution, StandardUniform};
use crate::rngs::Rng;

pub struct StandardNormal {}

pub struct Normal<T> {
    mean: T,
    std_dev: T,
}

macro_rules! normal_impl {
    ($fty:tt, $uty:ty) => {
        impl Normal<$fty> {
            pub fn new(mean: $fty, std_dev: $fty) -> Self {
                Self { mean, std_dev }
            }
        }

        impl Distribution<($fty, $fty)> for StandardNormal {
            type Backend = $uty;

            // Box-Muller transform
            fn sample<R>(&self, rng: &mut R) -> ($fty, $fty)
            where
                R: Rng<Self::Backend>,
            {
                let u1: $fty = rng.sample(&StandardUniform {});
                let u2: $fty = rng.sample(&StandardUniform {});

                let r = $fty::sqrt(-2.0 * $fty::ln(u1 + $fty::EPSILON / 2.0));
                let (sint, cost) = $fty::sin_cos(2.0 * core::$fty::consts::PI * u2);

                (r * cost, r * sint)
            }
        }

        impl Distribution<($fty, $fty)> for Normal<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> ($fty, $fty)
            where
                R: Rng<Self::Backend>,
            {
                let (z1, z2) = rng.sample(&StandardNormal {});

                (
                    $fty::mul_add(z1, self.std_dev, self.mean),
                    $fty::mul_add(z2, self.std_dev, self.mean),
                )
            }
        }
    };
}

normal_impl! { f32, u32 }
normal_impl! { f64, u64 }

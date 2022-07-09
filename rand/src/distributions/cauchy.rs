use crate::distributions::Distribution;
use crate::rngs::Rng;

pub struct Cauchy<T> {
    location: T,
    scale: T,
}

macro_rules! cauchy_impl {
    ($fty:tt) => {
        impl Cauchy<$fty> {
            pub fn new(location: $fty, scale: $fty) -> Self {
                Self { location, scale }
            }
        }

        impl Distribution<$fty> for Cauchy<$fty> {
            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<$fty>,
            {
                let p = rng.gen();
                self.location + self.scale * $fty::tan(core::$fty::consts::PI * (p - 0.5))
            }
        }
    };
}

cauchy_impl! { f32 }
cauchy_impl! { f64 }

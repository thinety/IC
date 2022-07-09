use crate::distributions::Distribution;

mod xoshiro128plus;
mod xoshiro128plusplus;
mod xoshiro256plus;
mod xoshiro256plusplus;

pub use xoshiro128plus::Xoshiro128Plus;
pub use xoshiro128plusplus::Xoshiro128PlusPlus;
pub use xoshiro256plus::Xoshiro256Plus;
pub use xoshiro256plusplus::Xoshiro256PlusPlus;

pub trait Rng<T>: Sized {
    fn gen(&mut self) -> T;

    fn sample<D>(&mut self, distribution: &D) -> T
    where
        D: Distribution<T>,
    {
        distribution.sample(self)
    }
}

macro_rules! float_rng_impl {
    ($fty:ty, $uty:ty, $total_bits:expr, $significant_bits:expr) => {
        impl<R> Rng<$fty> for R
        where
            R: Rng<$uty>,
        {
            fn gen(&mut self) -> $fty {
                let value = self.gen() >> ($total_bits - $significant_bits);

                let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

                scale * (value as $fty)
            }
        }
    };
}

float_rng_impl! { f32, u32, 32, 24 }
float_rng_impl! { f64, u64, 64, 53 }

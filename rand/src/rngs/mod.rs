use crate::distributions::Distribution;

mod xoshiro256plus;
mod xoshiro256plusplus;

pub use xoshiro256plus::Xoshiro256Plus;
pub use xoshiro256plusplus::Xoshiro256PlusPlus;

pub trait Rng: Sized {
    fn gen(&mut self) -> u64;

    fn sample<D, T>(&mut self, distribution: &D) -> T
    where
        D: Distribution<T>,
    {
        distribution.sample(self)
    }
}

use crate::rngs::Rng;

mod cauchy;
mod uniform;

pub use cauchy::Cauchy;
pub use uniform::Uniform;

pub trait Distribution<T> {
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng<T>;
}

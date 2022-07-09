use crate::rngs::Rng;

mod uniform;

pub use uniform::Uniform;

pub trait Distribution<T> {
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng<T>;
}

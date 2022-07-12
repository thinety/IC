use crate::rngs::Rng;

mod cauchy;
mod normal;
mod uniform;

pub use cauchy::{Cauchy, StandardCauchy};
pub use normal::{Normal, StandardNormal};
pub use uniform::{StandardUniform, Uniform};

pub trait Distribution<T> {
    type Backend;

    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng<Self::Backend>;
}

use crate::rngs::Rng;

mod cauchy;
mod normal;
mod uniform;

pub use cauchy::{Cauchy, StandardCauchy};
pub use normal::{Normal, StandardNormal};
pub use uniform::{
    StandardUniformClosedOpen, StandardUniformOpenClosed, StandardUniformOpenOpen, Uniform,
    UniformClosedOpen, UniformOpenClosed, UniformOpenOpen,
};

pub trait Distribution<T> {
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng;
}

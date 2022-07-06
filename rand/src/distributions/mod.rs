use crate::rngs::Rng;

mod uniform;

pub use uniform::{
    UniformClosedOpen, UniformClosedOpen01, UniformOpenClosed, UniformOpenClosed01,
    UniformOpenOpen, UniformOpenOpen01,
};

pub trait Distribution<T, U> {
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng<U>;
}

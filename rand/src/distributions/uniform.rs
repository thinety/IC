use crate::distributions::Distribution;
use crate::rngs::Rng;

pub struct Uniform<T> {
    start: T,
    range: T,
}

macro_rules! discrete_uniform_impl {
    ($uty:tt, $wty:ty, $shift:expr) => {
        impl Uniform<$uty> {
            pub fn new(start: $uty, end: $uty) -> Self {
                Self {
                    start,
                    range: end - start,
                }
            }
        }

        impl From<core::ops::Range<$uty>> for Uniform<$uty> {
            fn from(range: core::ops::Range<$uty>) -> Self {
                Self::new(range.start, range.end)
            }
        }

        impl Distribution<$uty> for Uniform<$uty> {
            type Backend = $uty;

            // https://arxiv.org/pdf/1805.10941.pdf
            fn sample<R>(&self, rng: &mut R) -> $uty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.gen();
                let mut m = (x as $wty) * (self.range as $wty);
                let mut l = m as $uty;

                if l < self.range {
                    let t = ($uty::MAX - self.range + 1) % self.range;

                    while l < t {
                        let x = rng.gen();
                        m = (x as $wty) * (self.range as $wty);
                        l = m as $uty;
                    }
                }

                self.start + (m >> $shift) as $uty
            }
        }
    };
}

discrete_uniform_impl! { u32, u64, 32 }
discrete_uniform_impl! { u64, u128, 64 }

pub struct StandardUniformClosedOpen {}

pub struct UniformClosedOpen<T> {
    start: T,
    range: T,
}

pub struct StandardUniformOpenClosed {}

pub struct UniformOpenClosed<T> {
    start: T,
    range: T,
}

pub struct StandardUniformOpenOpen {}

pub struct UniformOpenOpen<T> {
    start: T,
    range: T,
}

macro_rules! continuous_uniform_impl {
    ($fty:tt, $uty:ty, $total_bits:expr, $significant_bits:expr) => {
        impl UniformClosedOpen<$fty> {
            pub fn new(start: $fty, end: $fty) -> Self {
                Self {
                    start,
                    range: end - start,
                }
            }
        }

        impl From<core::ops::Range<$fty>> for UniformClosedOpen<$fty> {
            fn from(range: core::ops::Range<$fty>) -> Self {
                Self::new(range.start, range.end)
            }
        }

        impl UniformOpenClosed<$fty> {
            pub fn new(start: $fty, end: $fty) -> Self {
                Self {
                    start,
                    range: end - start,
                }
            }
        }

        impl From<core::ops::Range<$fty>> for UniformOpenClosed<$fty> {
            fn from(range: core::ops::Range<$fty>) -> Self {
                Self::new(range.start, range.end)
            }
        }

        impl UniformOpenOpen<$fty> {
            pub fn new(start: $fty, end: $fty) -> Self {
                Self {
                    start,
                    range: end - start,
                }
            }
        }

        impl From<core::ops::Range<$fty>> for UniformOpenOpen<$fty> {
            fn from(range: core::ops::Range<$fty>) -> Self {
                Self::new(range.start, range.end)
            }
        }

        impl Distribution<$fty> for StandardUniformClosedOpen {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let value = rng.gen() >> ($total_bits - $significant_bits);

                let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

                scale * (value as $fty)
            }
        }

        impl Distribution<$fty> for UniformClosedOpen<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.sample(&StandardUniformClosedOpen {});

                $fty::mul_add(x, self.range, self.start)
            }
        }

        impl Distribution<$fty> for StandardUniformOpenClosed {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let value = rng.gen() >> ($total_bits - $significant_bits);

                let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

                scale * ((value + 1) as $fty)
            }
        }

        impl Distribution<$fty> for UniformOpenClosed<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.sample(&StandardUniformOpenClosed {});

                $fty::mul_add(x, self.range, self.start)
            }
        }

        impl Distribution<$fty> for StandardUniformOpenOpen {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let value = rng.gen() >> ($total_bits - $significant_bits);

                let scale = 1.0 / (((1 as $uty) << $significant_bits) as $fty);

                scale * ((value | 1) as $fty)
            }
        }

        impl Distribution<$fty> for UniformOpenOpen<$fty> {
            type Backend = $uty;

            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng<Self::Backend>,
            {
                let x = rng.sample(&StandardUniformOpenOpen {});

                $fty::mul_add(x, self.range, self.start)
            }
        }
    };
}

continuous_uniform_impl! { f32, u32, 32, 24 }
continuous_uniform_impl! { f64, u64, 64, 53 }

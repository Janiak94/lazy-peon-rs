use self::types::VectorF32;
use rand::{Rng, SeedableRng};

use crate::types;

pub trait StepGenerator {
    fn step(&mut self) -> VectorF32;
}

pub trait KeyGenerator {
    fn next_key(&mut self) -> char;
}

pub struct RandomWalk<R> {
    step_size_px: f32,
    rng: R,
}

impl<R: rand::RngCore> RandomWalk<R> {
    pub fn new(step_size_px: f32, rng: R) -> Self {
        RandomWalk { step_size_px, rng }
    }
}

impl Default for RandomWalk<rand::rngs::StdRng> {
    fn default() -> Self {
        RandomWalk::new(1.0, rand::rngs::StdRng::from_entropy())
    }
}

impl<R> StepGenerator for RandomWalk<R>
where
    R: rand::RngCore,
{
    fn step(&mut self) -> VectorF32 {
        match self.rng.gen_range(0..4) {
            0 => VectorF32::new(0.0, self.step_size_px),
            1 => VectorF32::new(0.0, -self.step_size_px),
            2 => VectorF32::new(self.step_size_px, 0.0),
            3 => VectorF32::new(-self.step_size_px, 0.0),
            _ => unreachable!(),
        }
    }
}

pub struct RandomKeyGenerator<R> {
    rng: R,
}

impl Default for RandomKeyGenerator<rand::rngs::StdRng> {
    fn default() -> Self {
        RandomKeyGenerator::new(rand::rngs::StdRng::from_entropy())
    }
}

impl<R> RandomKeyGenerator<R>
where
    R: rand::RngCore,
{
    pub fn new(rng: R) -> Self {
        RandomKeyGenerator { rng }
    }
}

impl<R> KeyGenerator for RandomKeyGenerator<R>
where
    R: rand::RngCore,
{
    fn next_key(&mut self) -> char {
        self.rng.gen_range('a'..='z')
    }
}

use rand::prelude::*;

pub struct RandomGenerator {
    generator : ThreadRng
}

pub trait RandomGenerating {
    fn new() -> RandomGenerator;
    fn spit(&mut self, top: i32) -> i32;
    fn spit_range(&mut self, bottom: i32, top : i32) -> i32;
}

impl RandomGenerating for RandomGenerator{
    fn new() -> RandomGenerator {
        RandomGenerator {
            generator: thread_rng()
        }
    }

    fn spit(&mut self, top: i32) -> i32 {
        self.spit_range(0, top)
    }

    fn spit_range(&mut self, bottom: i32, top : i32) -> i32 {
        self.generator.gen_range(&bottom, &top) as i32
    }
}
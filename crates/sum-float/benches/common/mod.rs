use rand::distr::StandardUniform;
use rand::prelude::{RngExt, SeedableRng, StdRng};

pub(crate) fn prepare(len: usize) -> Vec<f32> {
    let rng = StdRng::seed_from_u64(0xbeef_cafe);
    rng.sample_iter(StandardUniform).take(len).collect()
}

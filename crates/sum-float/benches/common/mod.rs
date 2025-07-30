use rand::{Rng, SeedableRng, distr::StandardUniform};

pub(crate) fn prepare(len: usize) -> Vec<f32> {
    let rng = rand_chacha::ChaCha8Rng::seed_from_u64(0xbeef_cafe);
    rng.sample_iter(StandardUniform).take(len).collect()
}

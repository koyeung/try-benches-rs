use rand::distr::StandardUniform;
use rand::prelude::{Rng, SeedableRng, StdRng};

pub(crate) fn prepare() -> (Box<[u64]>, Box<[u64]>) {
    const SAMPLES: usize = 128;

    let mut rng = StdRng::seed_from_u64(0xbeef_cafe);

    let xs = (&mut rng)
        .sample_iter(StandardUniform)
        .take(SAMPLES)
        .collect::<Box<_>>();
    let ys = (&mut rng)
        .sample_iter(StandardUniform)
        .take(SAMPLES)
        .collect::<Box<_>>();

    (xs, ys)
}

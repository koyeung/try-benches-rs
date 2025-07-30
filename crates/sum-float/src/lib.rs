#![allow(internal_features)]
#![feature(core_intrinsics)]

// https://github.com/rust-lang/portable-simd/issues/426

use std::intrinsics::{fmul_algebraic, fmul_fast, fsub_algebraic, fsub_fast};

pub fn sum_scalar(data: &[f32], res: &mut [f32]) {
    let pos = [2000.0, 2000.0, 2000.0];
    let dir = [0.8, 0.6, 0.0];
    for i in 0..data.len() / 4 {
        let x = data[i * 4];
        let y = data[i * 4 + 1];
        let z = data[i * 4 + 2];
        res[i] = (x - pos[0]) * dir[0] + (y - pos[1]) * dir[1] + (z - pos[2]) * dir[2];
    }
}

/// # Safety
/// some description
pub unsafe fn sum_fast(data: &[f32], res: &mut [f32]) {
    unsafe {
        let pos = [2000.0, 2000.0, 2000.0];
        let dir = [0.8, 0.6, 0.0];
        for i in 0..data.len() / 4 {
            let x = data[i * 4];
            let y = data[i * 4 + 1];
            let z = data[i * 4 + 2];
            *res.get_unchecked_mut(i) = fmul_fast(fsub_fast(x, pos[0]), dir[0])
                + fmul_fast(fsub_fast(y, pos[1]), dir[1])
                + fmul_fast(fsub_fast(z, pos[2]), dir[2])
        }
    }
}

pub fn sum_fast_no_bound_checks(data: &[f32], res: &mut [f32]) {
    let pos = [2000.0, 2000.0, 2000.0];
    let dir = [0.8, 0.6, 0.0];
    for (r, chunk) in res.iter_mut().zip(data.chunks_exact(4)) {
        let x = chunk[0];
        let y = chunk[1];
        let z = chunk[2];
        *r = fmul_algebraic(fsub_algebraic(x, pos[0]), dir[0])
            + fmul_algebraic(fsub_algebraic(y, pos[1]), dir[1])
            + fmul_algebraic(fsub_algebraic(z, pos[2]), dir[2])
    }
}

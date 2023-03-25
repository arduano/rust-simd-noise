use crate::noise::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn ridge_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    simd_invoke!(S, {
        let mut amp = S::set1_pd(1.0);
        let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_1d::<S>(x, seed)));

        for _ in 1..octaves {
            x = S::mul_pd(x, lacunarity);
            amp = S::mul_pd(amp, gain);
            result = S::add_pd(
                result,
                S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_1d::<S>(x, seed))),
            );
        }

        result
    })
}

#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    simd_invoke!(S, {
        let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_2d::<S>(x, y, seed)));
        let mut amp = S::set1_pd(1.0);

        for _ in 1..octaves {
            x = S::mul_pd(x, lac);
            y = S::mul_pd(y, lac);
            amp = S::mul_pd(amp, gain);
            result = S::add_pd(
                result,
                S::fnmadd_pd(S::abs_pd(simplex_2d::<S>(x, y, seed)), amp, S::set1_pd(1.0)),
            );
        }

        result
    })
}

#[inline(always)]
pub unsafe fn ridge_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    simd_invoke!(S, {
        let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_3d::<S>(x, y, z, seed)));
        let mut amp = S::set1_pd(1.0);

        for _ in 1..octaves {
            x = S::mul_pd(x, lac);
            y = S::mul_pd(y, lac);
            z = S::mul_pd(z, lac);
            amp = S::mul_pd(amp, gain);
            result = S::add_pd(
                result,
                S::fnmadd_pd(
                    S::abs_pd(simplex_3d::<S>(x, y, z, seed)),
                    amp,
                    S::set1_pd(1.0),
                ),
            );
        }

        result
    })
}

#[inline(always)]
pub unsafe fn ridge_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    simd_invoke!(S, {
        let mut result = S::sub_pd(
            S::set1_pd(1.0),
            S::abs_pd(simplex_4d::<S>(x, y, z, w, seed)),
        );
        let mut amp = S::set1_pd(1.0);

        for _ in 1..octaves {
            x = S::mul_pd(x, lac);
            y = S::mul_pd(y, lac);
            z = S::mul_pd(z, lac);
            w = S::mul_pd(w, lac);
            amp = S::mul_pd(amp, gain);
            result = S::add_pd(
                result,
                S::sub_pd(
                    S::set1_pd(1.0),
                    S::abs_pd(S::mul_pd(simplex_4d::<S>(x, y, z, w, seed), amp)),
                ),
            );
        }

        result
    })
}

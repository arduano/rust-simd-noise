use simdeez::prelude::*;

pub struct Hash3d<S: Simd> {
    // Masks guiding dimension selection
    pub l8: S::Vf32,
    pub l4: S::Vf32,
    pub h12_or_14: S::Vf32,

    // Signs for the selected dimensions
    pub h1: S::Vf32,
    pub h2: S::Vf32,
}

impl<S> Hash3d<S>
where
    S: Simd,
{
    pub fn new(l8: S::Vf32, l4: S::Vf32, h12_or_14: S::Vf32, h1: S::Vf32, h2: S::Vf32) -> Self {
        Self {
            l8,
            l4,
            h12_or_14,
            h1,
            h2,
        }
    }
}
/// Compute hash values used by `grad3d` and `grad3d_dot`

#[inline(always)]
pub unsafe fn hash3d<S: Simd>(seed: i32, i: S::Vi32, j: S::Vi32, k: S::Vi32) -> Hash3d<S> {
    let mut hash = i ^ S::Vi32::set1(seed);
    hash = j ^ hash;
    hash = k ^ hash;
    hash = (hash * hash * 60493) * hash;
    hash = (hash >> 13) ^ hash;
    let hasha13 = hash & S::Vi32::set1(13);
    Hash3d::new(
        (hasha13.cmp_lt(S::Vi32::set1(8))).cast_f32(),
        (hasha13.cmp_lt(S::Vi32::set1(2))).cast_f32(),
        (hasha13.cmp_eq(S::Vi32::set1(12))).cast_f32(),
        (hash << 31).cast_f32(),
        ((hash & S::Vi32::set1(2)) << 30).cast_f32(),
    )
}

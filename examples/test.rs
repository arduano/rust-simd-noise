use simdnoise::{intrinsics::avx2, NoiseBuilder};

pub fn main() {
    let setting = NoiseBuilder::fbm_3d(64, 64, 64).wrap();
    let result = unsafe { avx2::get_3d_noise(&setting) };
    dbg!(result.0[0]);
}

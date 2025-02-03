use rand::{Rng, RngCore};

use crate::{data_square::DataSquare, lanczos_doubler::LanczosDoubler};

pub fn noise_2d<const SIZE_BITS: u32>(
    base_level: f32,
    amplitudes: &[f32],
    mut rng: impl RngCore,
) -> DataSquare<f32, SIZE_BITS> {
    assert_eq!(amplitudes.len(), SIZE_BITS as usize);

    let mut data = vec![base_level];

    let doubler = LanczosDoubler::<6>::new();

    for (i, amp) in amplitudes.iter().copied().enumerate() {
        assert_eq!(data.len(), 1 << (i * 2));
        data = doubler.upscale_2d(&data, i as u16);
        assert_eq!(data.len(), 1 << (i * 2 + 2));
        if amp.abs() > f32::EPSILON {
            for value in data.iter_mut() {
                *value += (rng.random::<f32>() - 0.5) * amp;
            }
        }
    }

    data.try_into().unwrap()
}

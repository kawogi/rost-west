use rand::Rng;

use crate::lanczos_doubler::LanczosDoubler;

pub fn noise_2d(base_level: f32, amplitudes: &[f32]) -> Vec<f32> {
    let mut rng = rand::thread_rng();

    let mut grid = vec![base_level];

    let doubler = LanczosDoubler::<6>::new();

    for (i, amp) in amplitudes.iter().copied().enumerate() {
        assert_eq!(grid.len(), 1 << (i * 2));
        grid = doubler.upscale_2d(&grid, i as u16);
        assert_eq!(grid.len(), 1 << (i * 2 + 2));
        if amp.abs() > f32::EPSILON {
            for value in grid.iter_mut() {
                *value += (rng.gen::<f32>() - 0.5) * amp;
            }
        }
    }

    grid
}

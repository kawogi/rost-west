use std::f32::consts::PI;

use crate::data_square::DataSquare;

/// through a limitation of Rust we cannot pass the kernel grade `A` directly, as we'd have to double it in the type declaration
/// instead we pass `WINDOW_LEN` which is `A * 2` and thus must be an even number
struct LanczosKernel<const WINDOW_LEN: usize>([f32; WINDOW_LEN]);

impl<const WINDOW_LEN: usize> LanczosKernel<WINDOW_LEN> {
    const WINDOW_OFFSET: usize = (WINDOW_LEN / 2) - 1;
    const A: f32 = (WINDOW_LEN / 2) as f32;

    /// `shift` describes where the center lobe of the kernel shall be place.
    /// This is the sub-pixel coordinate of the value to be retrieved by a sampler.
    /// It must be within 0.0..1.0, otherwise tha data captured by the window will be
    /// insufficient to produce an artifact-free result.
    fn new(shift: f32) -> Self {
        // example for lanczos3 (WINDOW_LEN = 6, WINDOW_OFFSET = 2, shift = 0.5)
        //
        //   0   0.5   1   1.5   2   2.5   3   3.5   4   4.5   5  (kernel array index)
        //  -2  -1.5  -1  -0.5   0   0.5   1   1.5   2   2.5   3  (input array index)
        //   +---------+---------+===>|----+---------+---------+  (===> shift)
        // -2.5 - 2  -1.5  -1  -0.5   0   0.5   1   1.5   2   2.5 (kernel parameter)
        //
        // if we want to compute a new value between 1 and 0 we need to fold:
        // pixel[-2] * lanczos(-2.5) (== kernel[0])
        // pixel[-1] * lanczos(-1.5) (== kernel[1])
        // pixel[ 0] * lanczos(-0.5) (== kernel[2])
        // pixel[ 1] * lanczos( 0.5) (== kernel[3])
        // pixel[ 2] * lanczos( 1.5) (== kernel[4])
        // pixel[ 3] * lanczos( 2.5) (== kernel[5])

        // example for lanczos2 (WINDOW_LEN = 4, WINDOW_OFFSET = 1, shift = 0.25)
        //
        //   0        0.5        1        1.5        2        2.5        3   (kernel array index)
        //  -1       -0.5        0        0.5        1        1.5        2   (input array index)
        //   +-------------------+===>|--------------+-------------------+   (===> shift)
        // -1.25     -1.75     -0.25  0   0.25      1.75      2.25      2.75 (kernel parameter)

        // if we want to compute a new value between 1 and 0 we need to fold:
        // pixel[-1] * lanczos(-1.25) (== kernel[0])
        // pixel[ 0] * lanczos(-0.25) (== kernel[1])
        // pixel[ 1] * lanczos( 0.75) (== kernel[2])
        // pixel[ 2] * lanczos( 1.75) (== kernel[3])

        let mut kernel = std::array::from_fn(|index| {
            Self::lanczos((index as i32 - Self::WINDOW_OFFSET as i32) as f32 - shift)
        });

        let normalize: f32 = kernel.iter().sum::<f32>().recip();
        kernel.iter_mut().for_each(|v| *v *= normalize);

        Self(kernel)
    }

    fn lanczos(x: f32) -> f32 {
        // kernel is symmetric around 0.0
        let x = x.abs();
        if x < f32::EPSILON {
            1.0
        } else if x >= Self::A {
            0.0
        } else {
            let pix = PI * x;
            Self::A * pix.sin() * (pix / Self::A).sin() / (pix * pix)
        }
    }

    fn fold(&self, source: &[f32; WINDOW_LEN]) -> f32 {
        self.0
            .iter()
            .zip(source)
            .map(|(kernel, source)| kernel * source)
            .sum()
    }

    fn iter(&self) -> impl Iterator<Item = f32> + '_ {
        self.0.iter().copied()
    }
}

pub(crate) struct LanczosDoubler<const WINDOW_LEN: usize> {
    kernel: LanczosKernel<WINDOW_LEN>,
}

impl<const WINDOW_LEN: usize> LanczosDoubler<WINDOW_LEN> {
    const WINDOW_OFFSET: usize = LanczosKernel::<WINDOW_LEN>::WINDOW_OFFSET;

    pub(crate) fn new() -> Self {
        Self {
            kernel: LanczosKernel::new(0.5),
        }
    }

    pub(crate) fn upscale_2d(&self, data: &[f32], size_bits_in: u16) -> Vec<f32> {
        let size_in = 1 << size_bits_in;
        assert_eq!(data.len(), size_in * size_in);
        let size_bits_out = size_bits_in + 1;
        let size_out = 1 << size_bits_out;
        let mask_in = size_in - 1;

        let mut result = vec![0.0; size_out * size_out];

        // resize vertically

        // fill every even row of result with original values (but spread out)
        for (row_out, row_in) in result
            // slice into rows
            .chunks_exact_mut(size_out)
            // only even rows
            .step_by(2)
            .zip(data.chunks_exact(size_in))
        {
            row_out
                .iter_mut()
                .step_by(2)
                .zip(row_in)
                .for_each(|(out, &value_in)| *out = value_in);
        }

        // fill every odd row and even column of result with interpolated values
        for (y_in, row_out) in result
            // slice into rows
            .chunks_exact_mut(size_out)
            // only odd rows
            .skip(1)
            .step_by(2)
            .enumerate()
        {
            // select input rows for filter
            let rows_in: [&[f32]; WINDOW_LEN] = std::array::from_fn(|i| {
                let y_in = (y_in + i).wrapping_sub(Self::WINDOW_OFFSET) & mask_in;
                let offset = y_in * size_in;
                &data[offset..][..size_in]
            });

            for (x_in, values_out) in row_out.chunks_exact_mut(2).enumerate() {
                // select input pixels
                let values = std::array::from_fn(|i| rows_in[i][x_in]);

                // only even columns
                values_out[0] = self.kernel.fold(&values);
                // values_out[1] will be filled by the horizontal stage
            }
        }

        // resize horizontally

        // fetch every row from intermediate result
        for row in result.chunks_exact_mut(size_out) {
            for x_in in 0..size_in {
                // select input pixels around `x_in`
                let values = std::array::from_fn(|i| {
                    let x_in = (x_in + i).wrapping_sub(Self::WINDOW_OFFSET) & mask_in;
                    row[x_in * 2]
                });
                // intersperse interpolated values into odd columns
                row[x_in * 2 + 1] = self.kernel.fold(&values);
            }
        }

        result
    }
}

pub struct LanczosSampler<const WINDOW_LEN: usize> {
    resolution_bits: u32,
    kernels: Box<[LanczosKernel<WINDOW_LEN>]>,
}

impl<const WINDOW_LEN: usize> LanczosSampler<WINDOW_LEN> {
    pub const WINDOW_OFFSET: usize = LanczosKernel::<WINDOW_LEN>::WINDOW_OFFSET;

    pub fn new(resolution_bits: u32) -> Self {
        let kernel_count = 1_u32 << resolution_bits;
        let kernels = (0..kernel_count)
            .map(|shift| LanczosKernel::new(shift as f32 / kernel_count as f32))
            .collect::<Box<[_]>>();
        assert_eq!(kernels.len(), kernel_count as usize);
        Self {
            resolution_bits,
            kernels,
        }
    }

    pub fn resolution_bits(&self) -> u32 {
        self.resolution_bits
    }

    pub fn get(&self, shift: u16, window: &[f32; WINDOW_LEN]) -> f32 {
        self.kernels[usize::from(shift) & (self.kernels.len() - 1)].fold(window)
    }

}

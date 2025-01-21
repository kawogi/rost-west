use std::f32::consts::PI;

/// through a limitation of Rust we cannot pass the kernel grade `A` directly, as we'd have to double it in the type declaration
/// instead we pass `WINDOW_LEN` which is `A * 2` and thus must be an even number
pub(crate) struct LanczosDoubler<const WINDOW_LEN: usize> {
    kernel: [f32; WINDOW_LEN],
}

impl<const WINDOW_LEN: usize> LanczosDoubler<WINDOW_LEN> {
    const WINDOW_OFFSET: usize = (WINDOW_LEN / 2) - 1;
    const A: f32 = (WINDOW_LEN / 2) as f32;

    pub(crate) fn new() -> Self {
        // example for lanczos3 (WINDOW_LEN = 6, WINDOW_OFFSET = 2)
        // array index
        // -2  -1.5  -1  -0.5   0   0.5   1   1.5   2   2.5   3
        //  +---------+---------+---------+---------+---------+
        // -2.5 -2  -1.5  -1  -0.5   0   0.5   1   1.5   2   2.5
        // lanczos kernel parameter
        // if we want to compute a new value between 1 and 0 we need to fold:
        // pixel[-2] * lanczos(-2.5) (== kernel[0])
        // pixel[-1] * lanczos(-1.5) (== kernel[1])
        // pixel[ 0] * lanczos(-0.5) (== kernel[2])
        // pixel[ 1] * lanczos( 0.5) (== kernel[3])
        // pixel[ 2] * lanczos( 1.5) (== kernel[4])
        // pixel[ 3] * lanczos( 2.5) (== kernel[5])

        let mut kernel = std::array::from_fn(|index| {
            Self::lanczos((index as i32 - Self::WINDOW_OFFSET as i32) as f32 - 0.5)
        });

        let normalize: f32 = kernel.iter().sum::<f32>().recip();
        kernel.iter_mut().for_each(|v| *v *= normalize);

        Self { kernel }
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

    fn get_value(&self, source: &[f32; WINDOW_LEN]) -> f32 {
        self.kernel
            .iter()
            .zip(source)
            .map(|(kernel, source)| kernel * source)
            .sum()
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
                values_out[0] = self.get_value(&values);
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
                row[x_in * 2 + 1] = self.get_value(&values);
            }
        }

        result
    }
}

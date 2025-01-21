use std::{f32::consts::PI, fs::OpenOptions};

use rand::Rng;

const MAP_BITS: u16 = 12;
const MAP_SIZE: u16 = 1 << MAP_BITS;
const MAP_MASK: u16 = MAP_SIZE - 1;
const MAP_USIZE: usize = MAP_SIZE as usize;

const COARSE_BITS: u16 = 12;
const COARSE_SIZE: u16 = 1 << COARSE_BITS;
const COARSE_MASK: u16 = COARSE_SIZE - 1;
const COARSE_USIZE: usize = COARSE_SIZE as usize;
const COARSE_MAP_SHIFT: u16 = MAP_BITS - COARSE_BITS;
const COARSE_MAP_FACTOR: u16 = 1 << COARSE_MAP_SHIFT;
const COARSE_MAP_MASK: u16 = COARSE_MAP_FACTOR - 1;
const COARSE_MAP_SCALE: f32 = 1.0 / COARSE_MAP_FACTOR as f32;

// bit levels
// 16 65536 size of the entire map
// 15 32768
// 14 16384
// 13  8192
// 12  4096 size of a continent
// 11  2048
// 10  1024
//  9   512
//  8   256
//  7   128
//  6    64
//  5    32
//  4    16 size of a luanti block smallest feature we can realistically store
//  3     8
//  2     4
//  1     2
//  0     1 single node level

#[derive(Default, Clone, Copy)]
struct Vertex {
    h: f32,
    dx: f32,
    dy: f32,
}

impl Vertex {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            h: rng.gen_range(-0.5..=0.5),
            dx: rng.gen_range(-0.5..=0.5),
            dy: rng.gen_range(-0.5..=0.5),
        }
    }
}

fn split_scale(value: u16, fract_bits: u16) -> (u16, f32) {
    let fract = 1 << fract_bits;
    let mask = fract - 1;
    (
        value >> fract_bits,
        f32::from(value & mask) / f32::from(fract),
    )
}

#[derive(Clone, Copy)]
struct CubicInterpolation {
    v1: f32,
    v2: f32,
    m1: f32,
    m2: f32,
}

// +--------+--------+--------+--------+--------+

/// through a limitation of Rust we cannot pass the kernel grade `A` directly, as we'd have to double it in the type declaration
/// instead we pass `WINDOW_LEN` which is `A * 2` and thus must be an even number
struct LanczosUpscaler<const WINDOW_LEN: usize, const SCALE: usize> {
    kernels: [[f32; WINDOW_LEN]; SCALE],
}

impl<const WINDOW_LEN: usize, const SCALE: usize> LanczosUpscaler<WINDOW_LEN, SCALE> {
    const WINDOW_OFFSET: usize = (WINDOW_LEN / 2) - 1;
    const A: f32 = (WINDOW_LEN / 2) as f32;

    fn new() -> Self {
        let lanczos = |x: f32| -> f32 {
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
        };

        let kernels = std::array::from_fn(|shift| {
            let shift = shift as f32 / SCALE as f32;
            std::array::from_fn(|index| {
                let x = (index as i32 - Self::WINDOW_OFFSET as i32) as f32 + shift;
                lanczos(x)
            })
        });

        Self { kernels }
    }

    fn get_value(&self, source: &[f32; WINDOW_LEN], shift: usize) -> f32 {
        self.kernels[shift]
            .iter()
            .zip(source)
            .map(|(kernel, source)| kernel * source)
            .sum()
    }

    /// scales the input in y direction and returns the interpolated data in transposed orientation (x and y flipped)
    fn upscale_y(&self, data: &[f32], width: usize, height: usize) -> Vec<f32> {
        assert_eq!(data.len(), width * height);

        let mut result = Vec::with_capacity(width * (height * SCALE));

        for (kernel_shift, kernel) in self.kernels.iter().enumerate() {
            for source_y in 0..height {
                let rows: [&[f32]; WINDOW_LEN] = std::array::from_fn(|i| {
                    &data[((source_y + height + i - Self::WINDOW_OFFSET) % height)..][..width]
                });
                let dest_x = source_y * SCALE + kernel_shift;
                for source_x in 0..width {
                    result[source_x * height * SCALE + dest_x] = kernel
                        .iter()
                        .zip(rows)
                        .map(|(&weight, row)| weight * row[source_x])
                        .sum();
                }
            }
        }

        result
    }

    /// scales the input in y direction and returns the interpolated data in transposed orientation (x and y flipped)
    fn upscale_x(&self, data: &[f32], width: usize, height: usize) -> Vec<f32> {
        assert_eq!(data.len(), width * height);

        let mut result = Vec::with_capacity((width * SCALE) * height);

        for (kernel_shift, kernel) in self.kernels.iter().enumerate() {
            for source_y in 0..height {
                let rows: [&[f32]; WINDOW_LEN] = std::array::from_fn(|i| {
                    &data[((source_y + height + i - Self::WINDOW_OFFSET) % height)..][..width]
                });
                let dest_x = source_y * SCALE + kernel_shift;
                for source_x in 0..width {
                    result[source_x * height * SCALE + dest_x] = kernel
                        .iter()
                        .zip(rows)
                        .map(|(&weight, row)| weight * row[source_x])
                        .sum();
                }
            }
        }

        result
    }
}

/// through a limitation of Rust we cannot pass the kernel grade `A` directly, as we'd have to double it in the type declaration
/// instead we pass `WINDOW_LEN` which is `A * 2` and thus must be an even number
struct LanczosDoubler<const WINDOW_LEN: usize> {
    kernel: [f32; WINDOW_LEN],
}

impl<const WINDOW_LEN: usize> LanczosDoubler<WINDOW_LEN> {
    const WINDOW_OFFSET: usize = (WINDOW_LEN / 2) - 1;
    const A: f32 = (WINDOW_LEN / 2) as f32;

    fn new() -> Self {
        let lanczos = |x: f32| -> f32 {
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
        };

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

        let kernel = std::array::from_fn(|index| {
            lanczos((index as i32 - Self::WINDOW_OFFSET as i32) as f32 - 0.5)
        });

        dbg!(kernel);

        Self { kernel }
    }

    fn get_value(&self, source: &[f32; WINDOW_LEN]) -> f32 {
        self.kernel
            .iter()
            .zip(source)
            .map(|(kernel, source)| kernel * source)
            .sum()
    }

    fn upscale_2d(&self, data: &[f32], size_bits_in: u16) -> Vec<f32> {
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

        // fill every odd row of result with interpolated values
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
                // intersperse interpolated values into off columns
                row[x_in * 2 + 1] = self.get_value(&values);
            }
        }

        result
    }
}

impl CubicInterpolation {
    fn new(v1: f32, v2: f32, m1: f32, m2: f32) -> Self {
        Self { v1, v2, m1, m2 }
    }

    fn get(self, x: f32) -> f32 {
        let Self { v1, v2, m1, m2 } = self;
        let xx = x * x;
        let xxx = x * x * x;
        (2.0 * (v1 - v2) + m2 + m1) * xxx + (3.0 * (v2 - v1) - 2.0 * m1 - m2) * xx + m1 * x + v1
    }

    fn derive(self, x: f32) -> f32 {
        let Self { v1, v2, m1, m2 } = self;
        let xx = x * x;
        (6.0 * (v1 - v2) + 3.0 * (m2 + m1)) * xx + (6.0 * (v2 - v1) - 4.0 * m1 - 2.0 * m2) * x + m1
    }

    fn get_derive(self, x: f32) -> (f32, f32) {
        let Self { v1, v2, m1, m2 } = self;
        let xx = x * x;
        let xxx = x * x * x;
        let d_v2v1 = v2 - v1;
        let s_m1m2 = m1 + m2;
        let s_2m1m2 = 2.0 * m1 + m2;
        let a = -2.0 * d_v2v1 + s_m1m2;
        let b = 3.0 * d_v2v1 - s_2m1m2;
        let y = a * xxx + b * xx + m1 * x + v1;
        let m = 3.0 * a * xx + 2.0 * b * x + m1;
        (y, m)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![Vertex::default(); COARSE_USIZE]; COARSE_USIZE];
    for row in &mut grid {
        for vertex in row {
            *vertex = Vertex::rand();
        }
    }
    for y in 0..COARSE_SIZE {
        let below = (y + COARSE_SIZE - 1) & COARSE_MASK;
        let above = (y + 1) & COARSE_MASK;
        for x in 0..COARSE_SIZE {
            let left = (x + COARSE_SIZE - 1) & COARSE_MASK;
            let right = (x + 1) & COARSE_MASK;
            let h_above = grid[above as usize][x as usize].h;
            let h_below = grid[below as usize][x as usize].h;
            let h_right = grid[y as usize][right as usize].h;
            let h_left = grid[y as usize][left as usize].h;
            grid[y as usize][x as usize].dy = (h_above - h_below) / f32::from(COARSE_SIZE * 2);
            grid[y as usize][x as usize].dx = (h_right - h_left) / f32::from(COARSE_SIZE * 2);
        }
    }

    let mut grid = vec![0.0];
    // rng.fill(grid.as_mut_slice());
    // grid.iter_mut().for_each(|value| *value -= 0.5);

    let doubler = LanczosDoubler::<6>::new();

    let amps = [
        0.0, // 32768
        0.0, // 16384
        0.0, // 8192
        1.0, // 4096
        1.0 / 2.0,
        1.0 / 4.0,
        1.0 / 8.0,
        1.0 / 16.0,
        1.0 / 32.0,
        1.0 / 64.0,
        1.0 / 128.0,
        1.0 / 256.0,
    ];
    assert_eq!(amps.len(), usize::from(COARSE_BITS));
    for (i, amp) in amps.iter().copied().enumerate() {
        assert_eq!(grid.len(), 1 << (i * 2));
        grid = doubler.upscale_2d(&grid, i as u16);
        assert_eq!(grid.len(), 1 << (i * 2 + 2));
        for value in grid.iter_mut() {
            *value += (rng.gen::<f32>() - 0.5) * amp;
        }
    }

    assert_eq!(grid.len(), 16_777_216);

    let data = grid
        .into_iter()
        .map(|v| ((v + 0.5) * 255.0).round() as u8)
        .collect::<Vec<_>>();

    // let grid = grid
    //     .chunks_exact(COARSE_USIZE)
    //     .map(|row| {
    //         row.iter()
    //             .map(|&h| Vertex {
    //                 h,
    //                 dx: 0.0,
    //                 dy: 0.0,
    //             })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();

    // let mut data = vec![0; MAP_USIZE * MAP_USIZE];
    // for y in 0..MAP_SIZE {
    //     let (grid_y0, yy) = split_scale(y, COARSE_MAP_SHIFT);
    //     let grid_y1 = (grid_y0 + 1) & COARSE_MASK;
    //     for x in 0..MAP_SIZE {
    //         let (grid_x0, xx) = split_scale(x, COARSE_MAP_SHIFT);
    //         let grid_x1 = (grid_x0 + 1) & COARSE_MASK;

    //         let bl = grid[usize::from(grid_y0)][usize::from(grid_x0)];
    //         let br = grid[usize::from(grid_y0)][usize::from(grid_x1)];
    //         let tl = grid[usize::from(grid_y1)][usize::from(grid_x0)];
    //         let tr = grid[usize::from(grid_y1)][usize::from(grid_x1)];

    //         // let (b_h, b_m) = CubicInterpolation::new(bl.h, bl.dx, br.h, br.dx).get_derive(xx);
    //         // let (t_h, t_m) = CubicInterpolation::new(tl.h, tl.dx, tr.h, tr.dx).get_derive(xx);
    //         // let (h, m) = CubicInterpolation::new(b_h, b_m, t_h, t_m).get_derive(yy);
    //         let b_h = bl.h + (br.h - bl.h) * xx;
    //         let t_h = tl.h + (tr.h - tl.h) * xx;
    //         let h = b_h + (t_h - b_h) * yy;

    //         let c = (h * 128.0 + 128.0).round() as u8;
    //         data[usize::from(y) << MAP_BITS | usize::from(x)] = c;
    //     }
    // }

    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("map.png")
        .unwrap();

    let mut encoder = png::Encoder::new(out_file, u32::from(MAP_SIZE), u32::from(MAP_SIZE));
    encoder.set_color(png::ColorType::Grayscale);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
    writer.finish().unwrap();
}

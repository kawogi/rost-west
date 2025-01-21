pub mod fractal_noise;
mod lanczos_doubler;

use core::f32;
use std::{f32::consts::PI, fs::OpenOptions};

use fractal_noise::noise_2d;
use glam::{I8Vec3, U8Vec3, Vec2, Vec3};
use lanczos_doubler::LanczosDoubler;
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
const ERROR: Vec3 = Vec3::new(255.0, 0.0, 0.0);
const DEEP_SEA: Vec3 = MID_SEA;
const MID_SEA: Vec3 = Vec3::new(5.2, 17.7, 22.1);
const SHALLOW_SEA: Vec3 = MID_SEA;
const COAST: Vec3 = Vec3::new(49.8 * 0.7, 47.1 * 0.7, 39.6 * 0.7);
const GRASS: Vec3 = Vec3::new(26.5, 33.4, 27.2);
const COAST_GRASS: Vec3 = Vec3::new(20.6, 29.7, 8.3);
const DIRT: Vec3 = Vec3::new(52.3, 49.1, 35.2);
const SNOW: Vec3 = Vec3::new(128.0, 128.0, 128.0);

// const ROCK: Vec3 = Vec3::new(0.7, 0.7, 0.7);

const GRADIENT: [(f32, Vec3); 8] = [
    (-1.0, DEEP_SEA),
    (-0.5, MID_SEA),
    (-0.1, SHALLOW_SEA),
    (0.0, COAST),
    (0.1, COAST_GRASS),
    (0.5, GRASS),
    (0.6, DIRT),
    (1.0, SNOW),
];

fn map_color(value: f32) -> Vec3 {
    let color = GRADIENT.windows(2).find_map(|pair| {
        let &[(a, color_a), (b, color_b)] = pair else {
            panic!();
        };
        (value != a && value <= b).then(|| {
            let x = (value - a) / (b - a);
            color_a.lerp(color_b, x)
        })
    });

    // let color = color.map(|color| color.lerp(SNOW, 0.5));

    color.unwrap_or(if value < 0.0 { ERROR } else { ERROR }) * 2.5
}

fn main() {
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

    let grid = noise_2d(0.0, &amps);

    assert_eq!(grid.len(), 16_777_216);

    let mut slope = Vec::with_capacity(grid.len());

    let offset = |x, y| (usize::from(y & MAP_MASK) << MAP_BITS) | usize::from(x & MAP_MASK);

    let mut i = 0;
    for y in 0..MAP_SIZE {
        let up = y.wrapping_sub(1) & MAP_MASK;
        let down = y.wrapping_add(1) & MAP_MASK;
        for x in 0..MAP_SIZE {
            let left = x.wrapping_sub(1) & MAP_MASK;
            let right = x.wrapping_add(1) & MAP_MASK;
            let dx = (grid[offset(right, y)] - grid[offset(left, y)]) / 2.0;
            let dy = (grid[offset(x, down)] - grid[offset(x, up)]) / 2.0;
            slope.push(Vec2::new(dx, dy));
            assert_eq!(i, offset(x, y));
            i += 1;
        }
    }

    let data = grid
        .into_iter()
        .zip(slope)
        .flat_map(|(height, slope)| {
            let slope = if height <= 0.0 {
                Vec2::new(0.0, 0.0)
            } else {
                slope
            };

            let normal = Vec3::new(-slope.x * 200.0, -slope.y * 200.0, 1.0).normalize();
            let light_direction = Vec3::new(-1.0, -1.0, 1.0).normalize();
            let light = normal.dot(light_direction);

            let light = (light * 0.5 + 0.5).clamp(0.0, 1.0);

            let color = map_color(height) * light;

            color.as_u8vec3().to_array()
        })
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
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
    writer.finish().unwrap();
}

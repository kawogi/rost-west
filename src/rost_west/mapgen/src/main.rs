pub mod data_square;
pub mod fractal_noise;
mod lanczos_doubler;

use core::f32;
use std::{f32::consts::PI, fs::OpenOptions};

use data_square::{DataSquare, GetWrapping, Sampler};
use fractal_noise::noise_2d;
use glam::{I8Vec3, U8Vec3, Vec2, Vec3};
use lanczos_doubler::{LanczosDoubler, LanczosSampler};
use rand::Rng;

type WorldCoordinate = u16;

const WORLD_BITS: u32 = WorldCoordinate::BITS;
const WORLD_SHIFT: u32 = 0; // WORLD_BITS - WORLD_BITS;
const WORLD_SIZE: u32 = 1 << WORLD_BITS;
const WORLD_MASK: WorldCoordinate = (WORLD_SIZE - 1) as WorldCoordinate;

const ELEVATION_MAP_BITS: u32 = 12;
const ELEVATION_MAP_SHIFT: u32 = WORLD_BITS - ELEVATION_MAP_BITS;
const ELEVATION_MAP_SIZE: WorldCoordinate = 1 << MAP_BITS;
const ELEVATION_MAP_MASK: WorldCoordinate = (ELEVATION_MAP_SIZE - 1) as WorldCoordinate;

const TEMPERATURE_MAP_BITS: u32 = 8;

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

pub const MEAN_TEMPERATURE: f32 = 15.1;
pub const TEMPERATURE_RANGE: f32 = 30.0;
// Temperature shift per elevation: 70 °C / 10_000 m

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
const MID_SEA: Vec3 = Vec3::new(5.2 * 0.7, 17.7 * 0.7, 22.1 * 0.7);
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
    let elevation_amps = [
        0.0, // map=2048, world=32768
        0.0, // map=1024, world=16384
        0.0, // map= 512, world= 8192
        1.0, // map= 256, world= 4096 continent
        1.0, // map= 128, world= 2048
        1.0, // map=  64, world= 1024
        1.0, // map=  32, world=  512
        1.0, // map=  16, world=  256
        1.0, // map=   8, world=  128
        1.0, // map=   4, world=   64
        1.0, // map=   2, world=   32
        1.0, // map=   1, world=   16 (block size)
    ];

    let expected_max_elevation = elevation_amps
        .iter()
        .copied()
        .fold(0.0, |height, amp| height * 2.0 + amp * 0.5);
    println!("expected_max_elevation: {expected_max_elevation}");

    let elevation_map = noise_2d::<ELEVATION_MAP_BITS>(0.0, &elevation_amps);

    let lanczos_sampler = LanczosSampler::<6>::new(u16::BITS - ELEVATION_MAP_BITS);
    let sampler = Sampler::new(&elevation_map, &lanczos_sampler);

    // let temperature_amps = [
    //     1.0, // 128
    //     0.0, // 64
    //     0.0, // 32
    //     0.0, // 16
    //     0.0, // 8
    //     0.0, // 4
    //     0.0, // 2
    //     0.0, // 1
    // ];

    // let expected_max_temperature = temperature_amps
    //     .iter()
    //     .copied()
    //     .fold(0.0, |height, amp| height * 2.0 + amp * 0.5);
    // println!("expected_max_temperature: {expected_max_temperature}");

    // let mut temperature_map = noise_2d::<TEMPERATURE_MAP_BITS>(0.0, &temperature_amps);

    // temperature_map.as_mut().iter_mut().for_each(|temperature| {
    //     *temperature =
    //         MEAN_TEMPERATURE + *temperature / expected_max_temperature * TEMPERATURE_RANGE
    // });

    let mut slope: DataSquare<Vec2, ELEVATION_MAP_BITS> = DataSquare::new(Vec2::default());

    let offset = |x, y| (usize::from(y & MAP_MASK) << MAP_BITS) | usize::from(x & MAP_MASK);

    let mut i = 0;
    for y in 0..ELEVATION_MAP_SIZE {
        for x in 0..ELEVATION_MAP_SIZE {
            let [top_left, top, top_right, left, _center, right, bottom_left, bottom, bottom_right] =
                elevation_map.get_3x3([x, y]);
            let left = top_left + left * 2.0 + bottom_left;
            let right = top_right + right * 2.0 + bottom_right;
            let top = top_left + top * 2.0 + top_right;
            let bottom = bottom_left + bottom * 2.0 + bottom_right;
            let dx = (right - left) / 8.0;
            let dy = (bottom - top) / 8.0;
            slope.set([x, y], Vec2::new(dx, dy));
            assert_eq!(i, offset(x, y));
            i += 1;
        }
    }

    let mut color_rgb = Vec::with_capacity(4096 * 4096 * 3);
    for y in 0..4096 {
        let world_y = y << 4;
        for x in 0..4096 {
            let world_x = x << 4;

            let elevation = sampler.sample([world_x, world_y]);
            // let slope = sampler.sample([world_x, world_y]);

            let slope = Vec2::new(0.0, 0.0);
            let light = 1.0;

            let color = map_color(elevation / expected_max_elevation) * light;

            color_rgb.extend_from_slice(&color.as_u8vec3().to_array());
        }
    }

    // let color_rgb = elevation_map
    //     .iter()
    //     .zip(slope.iter())
    //     .flat_map(|(&height, &slope)| {
    //         let slope = if height <= 0.0 {
    //             Vec2::new(0.0, 0.0)
    //         } else {
    //             slope
    //         };

    //         let normal = Vec3::new(-slope.x, -slope.y, 1.0).normalize();
    //         let light_direction = Vec3::new(-1.0, -1.0, 1.0).normalize();
    //         let light = normal.dot(light_direction);

    //         let light = (light * 0.5 + 0.5).clamp(0.0, 1.0);

    //         let color = map_color(height / expected_max_elevation) * light;

    //         color.as_u8vec3().to_array()
    //     })
    //     .collect::<Vec<_>>();

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
    writer.write_image_data(&color_rgb).unwrap();
    writer.finish().unwrap();
}

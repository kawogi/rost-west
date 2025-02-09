use std::{
    collections::{hash_map::Entry, HashMap},
    f32::consts::TAU,
    hash::Hash,
};

use glam::{I16Vec2, Vec2};
use mapgen::{
    data_series::{DataSeries, Sampler1d},
    data_square::{DataSquare, Sampler2d},
    lanczos_doubler::LanczosSampler,
};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use crate::MAP_BLOCKSIZE;

const BITS_PER_BLOCK: u32 = 4;
const ELEVATION_RESOLUTION: u32 = u16::BITS - BITS_PER_BLOCK;
const TEMPERATURE_RESOLUTION: u32 = u16::BITS - 8;

/// average temperature at sea level
pub(crate) const GROUND_TEMPERATURE: f32 = celsius_to_kelvin(14.0);

/// at which elevation snow is to be expected on average
pub(crate) const SNOW_ELEVATION: f32 = 64.0;

/// at what temperature snow is expected to occur
pub(crate) const SNOW_TEMPERATURE: f32 = celsius_to_kelvin(0.0);

/// rate at which the temperature falls with elevation (this value is negative and meant to be multiplied with y)
pub(crate) const TEMPERATURE_LAPSE_RATE: f32 =
    (SNOW_TEMPERATURE - GROUND_TEMPERATURE) / SNOW_ELEVATION;

const ELEVATION_AMPLITUDES: [f32; ELEVATION_RESOLUTION as usize] = [
    0.0,  // 32768
    0.0,  // 16384
    0.0,  // 8192
    0.0,  // 4096
    0.0,  // 2048
    0.0,  // 1024
    64.0, // 512
    32.0, // 256
    16.0, // 128
    8.0,  // 64
    4.0,  // 32
    2.0,  // 16
];

const TEMPERATURE_AMPLITUDES: [f32; TEMPERATURE_RESOLUTION as usize] = [
    0.0,  // 32768
    0.0,  // 16384
    0.0,  // 8192
    0.0,  // 4096
    0.0,  // 2048
    16.0, // 1024
    16.0, // 512
    16.0, // 256
];

pub(crate) struct NodeSurface {
    // y-coordinate of the surface level
    pub(crate) elevation: f32,
    // in Kelvin
    pub(crate) temperature: f32,
    // how much the elevation changes when moving from (x, y) → (x + 1, z + 1)
    pub(crate) slope: Vec2,
}

pub(crate) struct BlockSurface {
    nodes: [[NodeSurface; MAP_BLOCKSIZE as usize]; MAP_BLOCKSIZE as usize],
}
impl BlockSurface {
    #[inline]
    pub(crate) fn node(&self, [x, y]: [i16; 2]) -> &NodeSurface {
        &self.row(y)[(x & (MAP_BLOCKSIZE - 1)) as usize]
    }

    #[inline]
    pub(crate) fn row(&self, y: i16) -> &[NodeSurface; MAP_BLOCKSIZE as usize] {
        &self.nodes[(y & (MAP_BLOCKSIZE - 1)) as usize]
    }
}

pub(crate) struct Surface {
    elevation_map: DataSquare<f32, ELEVATION_RESOLUTION>,
    blocks: HashMap<I16Vec2, BlockSurface>,
    elevation_sampler: LanczosSampler<6>,
    temperature_map: DataSquare<f32, TEMPERATURE_RESOLUTION>,
    temperature_sampler: LanczosSampler<4>,
}

pub(crate) const fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}

pub(crate) const fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}

impl Surface {
    pub fn new(seed: u64) -> Self {
        let elevation_rng = Pcg64Mcg::seed_from_u64(seed.rotate_left(1));
        let temperature_rng = Pcg64Mcg::seed_from_u64(seed.rotate_left(2));

        let expected_elevation = ELEVATION_AMPLITUDES.iter().sum::<f32>();
        let mut elevation_map =
            mapgen::fractal_noise::noise_2d(0.0, &ELEVATION_AMPLITUDES, elevation_rng);

        let max_height = elevation_map
            .iter()
            .map(|y| y.abs())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();
        println!("expected: {expected_elevation}, max: {max_height}");

        let temperature_map = mapgen::fractal_noise::noise_2d(
            GROUND_TEMPERATURE,
            &TEMPERATURE_AMPLITUDES,
            temperature_rng,
        );

        let max_temperature = temperature_map
            .iter()
            .copied()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();
        let min_temperature = temperature_map
            .iter()
            .copied()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();
        println!(
            "temperature min: {min} °C, max: {max} °C, lapse rate {TEMPERATURE_LAPSE_RATE} K/node",
            min = kelvin_to_celsius(min_temperature),
            max = kelvin_to_celsius(max_temperature)
        );

        let offset_amplitudes = [
            0.0, // 512
            0.0, // 256
            0.0, // 128
            0.0, // 64
            1.0, // 32
            0.0, // 16
            0.0, // 8
            0.0, // 4
            0.0, // 2
        ];

        let mut offset_map: DataSeries<f32, 9> = DataSeries::new(0.0);
        offset_map.iter_mut().enumerate().for_each(|(x, value)| {
            let elevation = ((x << (u16::BITS - 9)) as i16) >> (u16::BITS - 9);
            // *value = rand::thread_rng().gen_range(-250.0..250.0); // -f32::from(x as i16);
            let alpha = (f32::from(elevation) / max_height * TAU * 16.0).sin() * 1.0;
            // let elevation = (f32::from(elevation) / max_height)
            //     .abs()
            //     .powf(2.0)
            //     .copysign(f32::from(elevation))
            //     * max_height;
            // *value = -f32::from(elevation as i16);
            *value = -alpha;
        });

        let off_sampler = LanczosSampler::<6>::new(7);
        let offset_sampler = Sampler1d::<9, 6>::new(&offset_map, &off_sampler);

        elevation_map.iter_mut().enumerate().for_each(|(index, h)| {
            let x = ((index << 4) as i16) >> 4;
            let y = ((index >> 12) as i16) >> 4;
            // let (x, y) = (index & 4095, (index >> 12) as u16);
            let offset = offset_map.get((*h) as i64 as i16 as u16);
            if x > 0 {
                *h += offset;
                // } else {
                // *h = 0.0;
            }
            // *h = (*h / 10.0).round() * 10.0;
            // let h = if h > 0.0 { h.powf(2.0) } else { h };
        });

        let elevation_sampler = LanczosSampler::<6>::new(16 - ELEVATION_RESOLUTION);
        let temperature_sampler = LanczosSampler::<4>::new(16 - TEMPERATURE_RESOLUTION);

        Self {
            blocks: HashMap::with_capacity(100_000),
            elevation_map,
            elevation_sampler,
            temperature_map,
            temperature_sampler,
        }
    }

    pub(crate) fn block_surface(&mut self, index: [i16; 2]) -> &BlockSurface {
        self.blocks.entry(index.into()).or_insert_with(|| {
            //
            let elevation = Sampler2d::new(&self.elevation_map, &self.elevation_sampler);
            let temperature = Sampler2d::new(&self.temperature_map, &self.temperature_sampler);
            // world coordinate of the block
            let [block_x, block_y] = [index[0] << BITS_PER_BLOCK, index[1] << BITS_PER_BLOCK];

            let nodes = std::array::from_fn(|y| {
                let world_y = block_y.wrapping_add(y as i16) as u16;
                std::array::from_fn(|x| {
                    let world_x = block_x.wrapping_add(x as i16) as u16;
                    NodeSurface {
                        elevation: elevation.sample([world_x, world_y]),
                        temperature: temperature.sample([world_x, world_y]),
                        slope: Vec2::new(0.0, 0.0),
                    }
                })
            });

            BlockSurface { nodes }
        })
    }
}

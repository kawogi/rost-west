use glam::{I16Vec3, IVec3};
use mapgen::{
    data_square::{DataSquare, GetWrapping, Sampler},
    lanczos_doubler::LanczosSampler,
};
use rand::Rng;

use crate::{
    ffi::{NodeDefManager, VoxelArea},
    materials::Materials,
    MapData, MapgenId, MAP_BLOCKSIZE,
};

pub(crate) struct Generator {
    id: MapgenId,
    materials: Option<Materials>,
    elevation: DataSquare<f32, 12>,
    sampler: LanczosSampler<6>,
}

// fn get_material(y: i16, h: i16, materials: &Materials) -> u16 {
//     match h {
//         ..-5 => materials.stone,
//         -5..-1 => materials.dirt,
//         -1 => materials.dirt_with_grass,
//         0.. => {
//             if y < 0 {
//                 materials.water_source
//             } else {
//                 materials.air
//             }
//         }
//     }
// }

fn get_material(y: i16, elevation: f32, materials: &Materials) -> u16 {
    let altitude = f32::from(y) - elevation;

    let mut rng = rand::thread_rng();
    // let y = f32::from(y);

    match (altitude, y) {
        // top layer
        (-1.0..0.0, y) => {
            let y = f32::from(y);
            if y < rng.gen_range(-1.0..=1.0) {
                materials.sand
            } else if y > 20.0 + rng.gen_range(-2.0..=2.0) {
                materials.dirt_with_snow
            } else {
                materials.dirt_with_grass
            }
        }
        // below top layer
        (-3.0..-1.0, _) => materials.dirt,
        (-10.0..-3.0, _) => materials.stone,
        // bottom layer
        (..-10.0, _) => materials.stone, // materials.lava_source,
        (_, ..0) => materials.water_source,
        (_, _) => materials.air,
    }
}

impl Generator {
    pub(crate) fn new(id: MapgenId) -> Self {
        let elevation_amplitudes = [
            0.0,   // 32768
            0.0,   // 16384
            0.0,   // 8192
            0.0,   // 4096
            0.0,   // 2048
            0.0,   // 1024
            128.0, // 512
            64.0,  // 256
            32.0,  // 128
            16.0,  // 64
            8.0,   // 32
            4.0,   // 16
        ];

        let offset_amplitudes = [
            0.0,   // 32768
            0.0,   // 16384
            0.0,   // 8192
            0.0,   // 4096
            0.0,   // 2048
            0.0,   // 1024
            128.0, // 512
            32.0,  // 256
            16.0,  // 128
            8.0,   // 64
            16.0,  // 32
            8.0,   // 16
        ];

        let expected_elevation = elevation_amplitudes.iter().sum::<f32>() / 2.0;
        let mut elevation_map = mapgen::fractal_noise::noise_2d(0.0, &elevation_amplitudes);

        let max_height = elevation_map
            .iter()
            .copied()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();
        println!("expected: {expected_elevation}, max: {max_height}");

        let offset_map: DataSquare<f32, 12> =
            mapgen::fractal_noise::noise_2d(0.0, &offset_amplitudes);

        elevation_map.iter_mut().enumerate().for_each(|(index, h)| {
            let offset = offset_map.get([
                index as u16,
                (*h / (4096.0 / expected_elevation)) as i64 as u16,
            ]);
            // *h += offset * 4.0;
            // *h = (*h / 10.0).round() * 10.0;
            // let h = if h > 0.0 { h.powf(2.0) } else { h };
        });

        let sampler = LanczosSampler::<6>::new(16 - elevation_amplitudes.len() as u32);

        Self {
            elevation: elevation_map,
            id,
            materials: None,
            sampler,
        }
    }

    /// center block is at [-2, -2, -2] → [2, 2, 2]
    /// - blockpos_min: lower corner block-coordinates of a chunk (5×5×5 blocks)
    /// - blockpos_max: higher corner block-coordinates of a chunk (5×5×5 blocks) (inclusive)
    pub fn make_chunk(
        &mut self,
        blockindex_min: I16Vec3,
        blockindex_max: I16Vec3,
        buffer_dimensions: IVec3,
        area: &VoxelArea,
        node_def_manager: &NodeDefManager,
        data: &mut [MapData],
    ) {
        let materials = self
            .materials
            .get_or_insert_with(|| Materials::load(node_def_manager));

        let nodepos_min = blockindex_min * MAP_BLOCKSIZE;
        let nodepos_max = blockindex_max * MAP_BLOCKSIZE;

        let node_min: I16Vec3 = blockindex_min * MAP_BLOCKSIZE;
        let node_max: I16Vec3 =
            (blockindex_max + I16Vec3::new(1, 1, 1)) * MAP_BLOCKSIZE - I16Vec3::new(1, 1, 1);

        println!(
            "make_chunk({id}) {blockindex_min} {blockindex_max} {nodepos_min} {nodepos_max} {buffer_dimensions} {data_len}",
            id = self.id,
            data_len = data.len(),
        );

        let elevation = Sampler::new(&self.elevation, &self.sampler);

        for z in node_min.z..=node_max.z {
            for y in node_min.y..=node_max.y {
                let mut i: usize = area.index(node_min.x, y, z) as usize;
                for x in node_min.x..=node_max.x {
                    let roughness = elevation.sample([(x as u16) << 4, (z as u16) << 4]) / 16.0;
                    let elevation = elevation.sample([x as u16, z as u16]) + roughness;
                    let material = get_material(y, elevation, materials);
                    data[i].id = material;
                    i += 1;
                }
            }
        }
    }
}

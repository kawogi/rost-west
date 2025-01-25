use glam::{I16Vec3, IVec3};
use mapgen::data_square::DataSquare;

use crate::{
    ffi::{NodeDefManager, VoxelArea},
    materials::Materials,
    MapData, MapgenId, MAP_BLOCKSIZE,
};

pub(crate) struct Generator {
    id: MapgenId,
    materials: Option<Materials>,
    elevation: DataSquare<f32, 12>,
}

fn get_material(y: i16, h: i16, materials: &Materials) -> u16 {
    match h {
        ..-5 => materials.stone,
        -5..-1 => materials.dirt,
        -1 => materials.dirt_with_grass,
        0.. => {
            if y < 0 {
                materials.water_source
            } else {
                materials.air
            }
        }
    }
}

// fn get_material(y: i16, h: i16, materials: &Materials) -> u16 {
//     if y < 0 {
//         materials.lava_source
//     } else {
//         materials.air
//     }
// }

impl Generator {
    pub(crate) fn new(id: MapgenId) -> Self {
        let amplitudes = [
            0.0, // 32768
            0.0, // 16384
            0.0, // 8192
            0.0, // 4096
            1.0, //
            1.0, //
            1.0, //
            1.0, //
            1.0, //
            1.0, //
            1.0, //
            1.0, //
        ];

        let elevation = mapgen::fractal_noise::noise_2d(0.0, &amplitudes);
        Self {
            elevation,
            id,
            materials: None,
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

        // let size = usize::from(size_x) * usize::from(size_y) * usize::from(size_z);
        // println!("z {size_x} {size_y} {size_z} {size} {}", data.len());
        println!(
            "make_chunk({id}) {blockindex_min} {blockindex_max} {nodepos_min} {nodepos_max} {buffer_dimensions} {data_len}",
            id = self.id,
            data_len = data.len(),
        );

        for z in node_min.z..=node_max.z {
            let row_offset = usize::from(((z >> 0) as u16) & 4095) << 12;
            for y in node_min.y..=node_max.y {
                let mut i: usize = area.index(node_min.x, y, z) as usize;
                for x in node_min.x..=node_max.x {
                    let col = usize::from(((x >> 0) as u16) & 4095);
                    let elevation = self.elevation[row_offset + col];
                    let h = y + ((elevation / 200.0).powi(3) * 200.0).round() as i16;
                    let material = get_material(y, h, materials);
                    data[i].id = material;
                    i += 1;
                }
            }
        }
    }
}

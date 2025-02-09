use glam::{I16Vec3, IVec3};
use rand::Rng;

use crate::{
    ffi::{NodeDefManager, VoxelArea},
    materials::Materials,
    surface::{celsius_to_kelvin, NodeSurface, Surface, TEMPERATURE_LAPSE_RATE},
    MapData, MapgenId, MAP_BLOCKSIZE,
};

pub(crate) struct Generator {
    id: MapgenId,
    materials: Option<Materials>,
    surface: Surface,
}

fn get_material(y: i16, surface: &NodeSurface, materials: &Materials) -> u16 {
    let mut rng = rand::rng();
    let elevation = surface.elevation;
    let temperature = surface.temperature + f32::from(y) * TEMPERATURE_LAPSE_RATE;
    let altitude = f32::from(y) - elevation;

    if elevation.is_sign_positive() {
        // above sea level
        match altitude {
            // top layer
            -1.0..0.0 => {
                let temperature =
                    temperature + rng.random_range(-3.0..3.0) * TEMPERATURE_LAPSE_RATE;
                if elevation < rng.random_range(0.0..2.0) {
                    materials.sand
                } else if temperature < celsius_to_kelvin(0.0) {
                    materials.dirt_with_snow
                } else if temperature > celsius_to_kelvin(30.0) {
                    materials.sand
                } else {
                    materials.dirt_with_grass
                }
            }
            // below top layer
            -3.0..-1.0 => materials.dirt,
            -10.0..-3.0 => materials.stone,
            // bottom layer
            ..-64.0 => materials.lava_source,
            _ => {
                let snow_level = (celsius_to_kelvin(-10.0) - temperature).max(0.0) * 0.3;
                if altitude < snow_level {
                    materials.snowblock
                } else {
                    materials.air
                }
            }
        }
    } else {
        // elevation is below sea level
        if y >= 0 {
            // everything above sea level
            materials.air
        } else if altitude < 0.0 {
            // everything below ground level
            materials.sand
        } else if y == -1 {
            // exactly on surface level
            if temperature < celsius_to_kelvin(-10.0) {
                materials.ice
            } else {
                materials.water_source
            }
        } else {
            materials.water_source
        }
    }
}

impl Generator {
    pub(crate) fn new(id: MapgenId) -> Self {
        let surface = Surface::new(0);

        Self {
            surface,
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
        map_data: &mut [MapData],
    ) {
        let materials = self
            .materials
            .get_or_insert_with(|| Materials::load(node_def_manager));

        println!(
            "make_chunk({id}) {blockindex_min} {blockindex_max} {buffer_dimensions} {data_len}",
            id = self.id,
            data_len = map_data.len(),
        );

        for blockindex_z in blockindex_min.z..=blockindex_max.z {
            let block_z = blockindex_z * MAP_BLOCKSIZE;
            for blockindex_x in blockindex_min.x..=blockindex_max.x {
                let block_x = blockindex_x * MAP_BLOCKSIZE;
                let surface = self.surface.block_surface([blockindex_x, blockindex_z]);
                for blockindex_y in blockindex_min.y..=blockindex_max.y {
                    let block_y = blockindex_y * MAP_BLOCKSIZE;
                    for z in 0..MAP_BLOCKSIZE {
                        let node_z = block_z + z;
                        for y in 0..MAP_BLOCKSIZE {
                            let node_y = block_y + y;
                            let row_start_index = area.index(block_x, node_y, node_z);
                            let map_data_row =
                                &mut map_data[row_start_index as usize..][..MAP_BLOCKSIZE as usize];
                            let surface_row = surface.row(z);
                            for (map_data, surface_node) in map_data_row.iter_mut().zip(surface_row)
                            {
                                map_data.id = get_material(node_y, surface_node, materials);
                            }
                        }
                    }
                }
            }
        }
    }
}

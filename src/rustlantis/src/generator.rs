use glam::{I16Vec3, IVec3};

use crate::{
    ffi::{NodeDefManager, VoxelArea},
    materials::Materials,
    MapData, MapgenId, MAP_BLOCKSIZE,
};

pub(crate) struct Generator {
    id: MapgenId,
    materials: Option<Materials>,
}

impl Generator {
    pub(crate) fn new(id: MapgenId) -> Self {
        Self {
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
            for y in node_min.y..=node_max.y {
                let material = match y {
                    ..-5 => materials.stone,
                    -5..-1 => materials.dirt,
                    -1 => materials.dirt_with_grass,
                    0.. => materials.air,
                };
                let mut i: usize = area.index(node_min.x, y, z) as usize;
                for _ in node_min.x..=node_max.x {
                    data[i].id = material;
                    i += 1;
                }
            }
        }
    }
}

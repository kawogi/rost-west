mod generator;
mod materials;
mod surface;

use std::{
    collections::{hash_map::Entry, HashMap},
    mem::transmute,
    sync::Mutex,
    thread,
};

use ffi::{NodeDefManager, VoxelArea};
use generator::Generator;
use glam::{I16Vec3, IVec3};
use lazy_static::lazy_static;
use rand::Rng;

/// Dimension of a MapBlock
const MAP_BLOCKSIZE: i16 = 16;

#[cxx::bridge]
mod ffi {
    // #[namespace = "voxel"]

    unsafe extern "C++" {
        include!("voxel.h");
        type VoxelArea;

        // s32 index(s16 x, s16 y, s16 z) const
        fn index(self: &VoxelArea, x: i16, y: i16, z: i16) -> i32;
        // // std::array<s32, 3> get_extent()
        // fn get_extent(self: &VoxelArea) -> [i32; 3];

    }

    // unsafe extern "C++" {
    //     include!("emerge.h");
    //     type BlockMakeData;

    // }

    // unsafe extern "C++" {
    //     include!("map.h");
    //     type MMVManip;

    // }

    unsafe extern "C++" {
        include!("nodedef.h");
        type NodeDefManager;

        // 	bool getId(const std::string &name, content_t &result) const;
        fn getId(&self, name: &CxxString, result: &mut u16) -> bool;

    }

    #[namespace = "rost_west"]
    extern "Rust" {
        fn mapgen_new() -> u32;
        fn mapgen_destroy(mapgen_id: u32) -> bool;
        fn mapgen_make_chunk(
            mapgen_id: u32,
            blockpos_min: [i16; 3],
            blockpos_max: [i16; 3],
            extent: [i32; 3],
            area: &VoxelArea,
            node_def_manager: &NodeDefManager,
            data: &mut [u32],
        ) -> bool;
    }
}

type MapgenId = u32;

lazy_static! {
    static ref MANAGER: Mutex<MapgenManager> = Mutex::new(MapgenManager::default());
}

fn mapgen_new() -> MapgenId {
    MANAGER.lock().unwrap().new_mapgen()
}

fn mapgen_destroy(mapgen_id: MapgenId) -> bool {
    MANAGER.lock().unwrap().destroy_mapgen(mapgen_id)
}

fn mapgen_make_chunk(
    mapgen_id: MapgenId,
    blockpos_min: [i16; 3],
    blockpos_max: [i16; 3],
    extent: [i32; 3],
    area: &VoxelArea,
    node_def_manager: &NodeDefManager,
    data: &mut [u32],
) -> bool {
    MANAGER.lock().unwrap().make_chunk(
        mapgen_id,
        blockpos_min,
        blockpos_max,
        extent,
        area,
        node_def_manager,
        data,
    )
}

#[derive(Default)]
struct MapgenManager {
    generators: HashMap<MapgenId, Generator>,
}

impl MapgenManager {
    fn new_mapgen(&mut self) -> MapgenId {
        loop {
            let id = rand::thread_rng().gen();
            if let Entry::Vacant(entry) = self.generators.entry(id) {
                entry.insert(Generator::new(id));

                println!("created mapgen #{id} {:?}", thread::current());
                return id;
            }
        }
    }

    fn destroy_mapgen(&mut self, id: MapgenId) -> bool {
        println!("destroying mapgen #{id} {:?}", thread::current());
        self.generators.remove(&id).is_some()
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "TODO maybe create a MakeChunkData-type"
    )]
    fn make_chunk(
        &mut self,
        mapgen_id: MapgenId,
        blockpos_min: [i16; 3],
        blockpos_max: [i16; 3],
        extent: [i32; 3],
        area: &VoxelArea,
        node_def_manager: &NodeDefManager,
        data: &mut [u32],
    ) -> bool {
        let Some(generator) = self.generators.get_mut(&mapgen_id) else {
            println!(
                "make_chunk: unknown generator #{mapgen_id} {:?}",
                thread::current()
            );
            return false;
        };
        println!("make_chunk #{mapgen_id} {:?}", thread::current());

        let blockpos_min = I16Vec3::from(blockpos_min);
        let blockpos_max = I16Vec3::from(blockpos_max);
        let extent = IVec3::from(extent);
        let data: &mut [MapData] = unsafe { transmute(data) };

        generator.make_chunk(
            blockpos_min,
            blockpos_max,
            extent,
            area,
            node_def_manager,
            data,
        );
        true
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MapData {
    id: u16,
    d1: u8,
    d2: u8,
}

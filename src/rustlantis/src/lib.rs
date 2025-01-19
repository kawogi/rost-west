// use std::fmt;

// TODO rename to Rost-West

mod generator;
mod materials;

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
// use ffi::VoxelArea;
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
        // std::array<s32, 3> get_extent()
        fn get_extent(self: &VoxelArea) -> [i32; 3];

    }

    unsafe extern "C++" {
        include!("emerge.h");
        type BlockMakeData;

    }

    unsafe extern "C++" {
        include!("map.h");
        type MMVManip;

    }

    unsafe extern "C++" {
        include!("nodedef.h");
        type NodeDefManager;

        // 	bool getId(const std::string &name, content_t &result) const;
        fn getId(&self, name: &CxxString, result: &mut u16) -> bool;

    }

    // #[namespace = "rustlantis"]
    // extern "Rust" {
    //     type I16Vec3;
    // }

    // type Settings;
    // type MMVManip;
    // type NodeDefManager;
    // type Biome;
    // type BiomeGen;
    // type BiomeParams;
    // type BiomeManager;
    // type EmergeParams;
    // type EmergeManager;
    // type MapBlock;
    // type VoxelManipulator;
    // type BlockMakeData;
    // type VoxelArea;
    // type Map;

    // type Person;

    // fn get_name(person: &Person) -> &CxxString;
    // fn make_person() -> UniquePtr<Person>;
    // fn is_black(self: &Color) -> bool;
    // }

    //     #[namespace = "shared"]
    //     struct Color {
    //         r: u8,
    //         g: u8,
    //         b: u8,
    //     }

    //     #[namespace = "shared"]
    //     struct SharedThing {
    //         points: Box<Points>,
    //         persons: UniquePtr<Person>,
    //         pixels: Vec<Color>,
    //     }

    //     unsafe extern "C++" {
    //         include!("cpp_part.h");
    //         type Person;

    //         fn get_name(person: &Person) -> &CxxString;
    //         fn make_person() -> UniquePtr<Person>;
    //         fn is_black(self: &Color) -> bool;
    //     }

    //     #[namespace = "rustlantis"]
    //     extern "Rust" {
    //         type Points;
    //         fn print_shared_thing(points: &SharedThing);
    //         fn make_shared_thing() -> SharedThing;
    //         fn rust_echo(val: i32) -> i32;
    //     }

    //     #[namespace = "shared"]
    //     extern "Rust" {
    //         fn is_white(self: &Color) -> bool;
    //     }
    #[namespace = "rustlantis"]
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

    #[namespace = "rustlantis"]
    extern "Rust" {
        // fn make_chunk(area: &VoxelArea, data: &mut [u32]);
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

// #[derive(Debug)]
// pub struct Points {
//     x: Vec<u8>,
//     y: Vec<u8>,
// }

// impl ffi::Color {
//     pub fn white() -> Self {
//         Self {
//             r: 255,
//             g: 255,
//             b: 255,
//         }
//     }

//     pub fn black() -> Self {
//         Self { r: 0, g: 0, b: 0 }
//     }

//     pub fn is_white(&self) -> bool {
//         self.r == 255 && self.g == 255 && self.b == 255
//     }
// }

// impl fmt::Debug for ffi::Color {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Color")
//             .field("r", &self.r)
//             .field("g", &self.g)
//             .field("b", &self.b)
//             .finish()
//     }
// }

// fn print_shared_thing(thing: &ffi::SharedThing) {
//     println!("{:#?}", thing.points);
//     println!(
//         "Pixel 0 is white: {}, pixel is black: {}",
//         thing.pixels[0].is_white(),
//         thing.pixels[1].is_black()
//     );
//     println!("{:#?}", ffi::get_name(thing.persons.as_ref().unwrap()));
// }

// fn make_shared_thing() -> ffi::SharedThing {
//     ffi::SharedThing {
//         points: Box::new(Points {
//             x: vec![1, 2, 3],
//             y: vec![4, 5, 6],
//         }),
//         persons: ffi::make_person(),
//         pixels: vec![ffi::Color::white(), ffi::Color::black()],
//     }
// }

// #[inline(always)]
// fn rust_echo(val: i32) -> i32 {
//     val
// }

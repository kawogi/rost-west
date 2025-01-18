// use std::fmt;

// TODO rename to Rost-West

use core::slice;
use std::mem::transmute;

use ffi::VoxelArea;
// use ffi::VoxelArea;
use rand::Rng;

/*
    A solid walkable node with the texture unknown_node.png.

    For example, used on the client to display unregistered node IDs
    (instead of expanding the vector of node definitions each time
    such a node is received).
*/
const CONTENT_UNKNOWN: u16 = 125;

/*
    The common material through which the player can walk and which
    is transparent to light
*/
const CONTENT_AIR: u16 = 126;

/*
    Ignored node.

    Unloaded chunks are considered to consist of this. Several other
    methods return this when an error occurs. Also, during
    map generation this means the node has not been set yet.

    Doesn't create faces with anything and is considered being
    out-of-map in the game map.
*/
const CONTENT_IGNORE: u16 = 127;

#[cxx::bridge]
mod ffi {
    // #[namespace = "voxel"]
    unsafe extern "C++" {
        include!("voxel.h");

        type VoxelArea;

        // s32 index(s16 x, s16 y, s16 z) const
        fn index(self: &VoxelArea, x: i16, y: i16, z: i16) -> i32;

    }

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
        // fn make_chunk(area: &VoxelArea, data: &mut [u32]);
        fn make_chunk(area: &VoxelArea, data: &mut [u32]);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MapData {
    id: u16,
    d1: u8,
    d2: u8,
}

pub fn make_chunk(area: &VoxelArea, data: &mut [u32]) {
    // let size = usize::from(size_x) * usize::from(size_y) * usize::from(size_z);
    // println!("z {size_x} {size_y} {size_z} {size} {}", data.len());
    println!("area {}", area.index(0, 0, 0));

    let data: &mut [MapData] = unsafe { transmute(data) };

    for (i, data) in data.iter_mut().enumerate() {
        if rand::thread_rng().gen_bool(0.5) {
            data.id = rand::thread_rng().gen::<u8>() as u16;
        }
    }
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

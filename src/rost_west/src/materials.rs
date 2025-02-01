use cxx::let_cxx_string;

use crate::ffi::NodeDefManager;

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

// stone
const BASENODE_STONE: &str = "basenodes:stone";
const BASENODE_DESERT_STONE: &str = "basenodes:desert_stone";
const BASENODE_COBBLE: &str = "basenodes:cobble";
const BASENODE_MOSSYCOBBLE: &str = "basenodes:mossycobble";

// dirt
const BASENODE_DIRT: &str = "basenodes:dirt";
const BASENODE_DIRT_WITH_GRASS: &str = "basenodes:dirt_with_grass";
const BASENODE_DIRT_WITH_SNOW: &str = "basenodes:dirt_with_snow";

// sand
const BASENODE_SAND: &str = "basenodes:sand";
const BASENODE_DESERT_SAND: &str = "basenodes:desert_sand";
const BASENODE_GRAVEL: &str = "basenodes:gravel";

// trees
const BASENODE_TREE: &str = "basenodes:tree";
const BASENODE_LEAVES: &str = "basenodes:leaves";
const BASENODE_JUNGLETREE: &str = "basenodes:jungletree";
const BASENODE_JUNGLELEAVES: &str = "basenodes:jungleleaves";
const BASENODE_PINE_TREE: &str = "basenodes:pine_tree";
const BASENODE_PINE_NEEDLES: &str = "basenodes:pine_needles";

// fluids
const BASENODE_WATER_SOURCE: &str = "basenodes:water_source";
const BASENODE_WATER_FLOWING: &str = "basenodes:water_flowing";
const BASENODE_RIVER_WATER_SOURCE: &str = "basenodes:river_water_source";
const BASENODE_RIVER_WATER_FLOWING: &str = "basenodes:river_water_flowing";
const BASENODE_LAVA_FLOWING: &str = "basenodes:lava_flowing";
const BASENODE_LAVA_SOURCE: &str = "basenodes:lava_source";

// plants
const BASENODE_JUNGLEGRASS: &str = "basenodes:junglegrass";
const BASENODE_APPLE: &str = "basenodes:apple";

// snow
const BASENODE_ICE: &str = "basenodes:ice";
const BASENODE_SNOW: &str = "basenodes:snow";
const BASENODE_SNOWBLOCK: &str = "basenodes:snowblock";

// nimap->set(0, "default:stone");
// nimap->set(2, "default:water_flowing");
// nimap->set(3, "default:torch");
// nimap->set(9, "default:water_source");
// nimap->set(14, "default:sign_wall");
// nimap->set(15, "default:chest");
// nimap->set(16, "default:furnace");
// nimap->set(17, "default:chest_locked");
// nimap->set(21, "default:fence_wood");
// nimap->set(30, "default:rail");
// nimap->set(31, "default:ladder");
// nimap->set(32, "default:lava_flowing");
// nimap->set(33, "default:lava_source");
// nimap->set(0x800, "default:dirt_with_grass");
// nimap->set(0x801, "default:tree");
// nimap->set(0x802, "default:leaves");
// nimap->set(0x803, "default:dirt_with_grass_footsteps");
// nimap->set(0x804, "default:mese");
// nimap->set(0x805, "default:dirt");
// nimap->set(0x806, "default:cloud");
// nimap->set(0x807, "default:coalstone");
// nimap->set(0x808, "default:wood");
// nimap->set(0x809, "default:sand");
// nimap->set(0x80a, "default:cobble");
// nimap->set(0x80b, "default:steelblock");
// nimap->set(0x80c, "default:glass");
// nimap->set(0x80d, "default:mossycobble");
// nimap->set(0x80e, "default:gravel");
// nimap->set(0x80f, "default:sandstone");
// nimap->set(0x810, "default:cactus");
// nimap->set(0x811, "default:brick");
// nimap->set(0x812, "default:clay");
// nimap->set(0x813, "default:papyrus");
// nimap->set(0x814, "default:bookshelf");
// nimap->set(0x815, "default:jungletree");
// nimap->set(0x816, "default:junglegrass");
// nimap->set(0x817, "default:nyancat");
// nimap->set(0x818, "default:nyancat_rainbow");
// nimap->set(0x819, "default:apple");
// nimap->set(0x820, "default:sapling");
// nimap->set(CONTENT_IGNORE, "ignore");
// nimap->set(CONTENT_AIR, "air");

pub(crate) struct Materials {
    // hardcoded
    #[allow(dead_code, reason = "those are not going away anytime soon")]
    pub(crate) unknown: u16,
    #[allow(dead_code, reason = "those are not going away anytime soon")]
    pub(crate) air: u16,
    #[allow(dead_code, reason = "those are not going away anytime soon")]
    pub(crate) ignore: u16,
    // basenodes
    pub(crate) stone: u16,
    pub(crate) desert_stone: u16,
    pub(crate) dirt_with_grass: u16,
    pub(crate) dirt_with_snow: u16,
    pub(crate) dirt: u16,
    pub(crate) sand: u16,
    pub(crate) desert_sand: u16,
    pub(crate) gravel: u16,
    pub(crate) junglegrass: u16,
    pub(crate) tree: u16,
    pub(crate) leaves: u16,
    pub(crate) jungletree: u16,
    pub(crate) jungleleaves: u16,
    pub(crate) pine_tree: u16,
    pub(crate) pine_needles: u16,
    pub(crate) water_source: u16,
    pub(crate) water_flowing: u16,
    pub(crate) river_water_source: u16,
    pub(crate) river_water_flowing: u16,
    pub(crate) lava_flowing: u16,
    pub(crate) lava_source: u16,
    pub(crate) cobble: u16,
    pub(crate) mossycobble: u16,
    pub(crate) apple: u16,
    pub(crate) ice: u16,
    pub(crate) snow: u16,
    pub(crate) snowblock: u16,
}

impl Materials {
    pub(crate) fn load(node_def_manager: &NodeDefManager) -> Self {
        let mut result = Self {
            unknown: CONTENT_UNKNOWN,
            air: CONTENT_AIR,
            ignore: CONTENT_IGNORE,
            // basenodes
            stone: CONTENT_UNKNOWN,
            desert_stone: CONTENT_UNKNOWN,
            dirt_with_grass: CONTENT_UNKNOWN,
            dirt_with_snow: CONTENT_UNKNOWN,
            dirt: CONTENT_UNKNOWN,
            sand: CONTENT_UNKNOWN,
            desert_sand: CONTENT_UNKNOWN,
            gravel: CONTENT_UNKNOWN,
            junglegrass: CONTENT_UNKNOWN,
            tree: CONTENT_UNKNOWN,
            leaves: CONTENT_UNKNOWN,
            jungletree: CONTENT_UNKNOWN,
            jungleleaves: CONTENT_UNKNOWN,
            pine_tree: CONTENT_UNKNOWN,
            pine_needles: CONTENT_UNKNOWN,
            water_source: CONTENT_UNKNOWN,
            water_flowing: CONTENT_UNKNOWN,
            river_water_source: CONTENT_UNKNOWN,
            river_water_flowing: CONTENT_UNKNOWN,
            lava_flowing: CONTENT_UNKNOWN,
            lava_source: CONTENT_UNKNOWN,
            cobble: CONTENT_UNKNOWN,
            mossycobble: CONTENT_UNKNOWN,
            apple: CONTENT_UNKNOWN,
            ice: CONTENT_UNKNOWN,
            snow: CONTENT_UNKNOWN,
            snowblock: CONTENT_UNKNOWN,
        };

        // let_cxx_string!(stone = BASENODE_STONE);
        let_cxx_string!(stone = "mcl_core:stone");
        node_def_manager.getId(&stone, &mut result.stone);

        let_cxx_string!(desert_stone = BASENODE_DESERT_STONE);
        node_def_manager.getId(&desert_stone, &mut result.desert_stone);

        // let_cxx_string!(dirt_with_grass = BASENODE_DIRT_WITH_GRASS);
        let_cxx_string!(dirt_with_grass = "mcl_core:dirt_with_grass");
        node_def_manager.getId(&dirt_with_grass, &mut result.dirt_with_grass);

        // let_cxx_string!(dirt_with_snow = BASENODE_DIRT_WITH_SNOW);
        let_cxx_string!(dirt_with_snow = "mcl_core:dirt_with_grass_snow");
        node_def_manager.getId(&dirt_with_snow, &mut result.dirt_with_snow);

        // let_cxx_string!(dirt = BASENODE_DIRT);
        let_cxx_string!(dirt = "mcl_core:dirt");
        node_def_manager.getId(&dirt, &mut result.dirt);

        // let_cxx_string!(sand = BASENODE_SAND);
        let_cxx_string!(sand = "mcl_core:sand");
        node_def_manager.getId(&sand, &mut result.sand);

        let_cxx_string!(desert_sand = BASENODE_DESERT_SAND);
        node_def_manager.getId(&desert_sand, &mut result.desert_sand);

        let_cxx_string!(gravel = BASENODE_GRAVEL);
        node_def_manager.getId(&gravel, &mut result.gravel);

        let_cxx_string!(junglegrass = BASENODE_JUNGLEGRASS);
        node_def_manager.getId(&junglegrass, &mut result.junglegrass);

        let_cxx_string!(tree = BASENODE_TREE);
        node_def_manager.getId(&tree, &mut result.tree);

        let_cxx_string!(leaves = BASENODE_LEAVES);
        node_def_manager.getId(&leaves, &mut result.leaves);

        let_cxx_string!(jungletree = BASENODE_JUNGLETREE);
        node_def_manager.getId(&jungletree, &mut result.jungletree);

        let_cxx_string!(jungleleaves = BASENODE_JUNGLELEAVES);
        node_def_manager.getId(&jungleleaves, &mut result.jungleleaves);

        let_cxx_string!(pine_tree = BASENODE_PINE_TREE);
        node_def_manager.getId(&pine_tree, &mut result.pine_tree);

        let_cxx_string!(pine_needles = BASENODE_PINE_NEEDLES);
        node_def_manager.getId(&pine_needles, &mut result.pine_needles);

        // let_cxx_string!(water_source = BASENODE_WATER_SOURCE);
        let_cxx_string!(water_source = "mcl_core:water_source");
        node_def_manager.getId(&water_source, &mut result.water_source);

        let_cxx_string!(water_flowing = BASENODE_WATER_FLOWING);
        node_def_manager.getId(&water_flowing, &mut result.water_flowing);

        let_cxx_string!(river_water_source = BASENODE_RIVER_WATER_SOURCE);
        node_def_manager.getId(&river_water_source, &mut result.river_water_source);

        let_cxx_string!(river_water_flowing = BASENODE_RIVER_WATER_FLOWING);
        node_def_manager.getId(&river_water_flowing, &mut result.river_water_flowing);

        let_cxx_string!(lava_flowing = BASENODE_LAVA_FLOWING);
        node_def_manager.getId(&lava_flowing, &mut result.lava_flowing);

        // let_cxx_string!(lava_source = BASENODE_LAVA_SOURCE);
        let_cxx_string!(lava_source = "mcl_core:lava_source");
        node_def_manager.getId(&lava_source, &mut result.lava_source);

        let_cxx_string!(cobble = BASENODE_COBBLE);
        node_def_manager.getId(&cobble, &mut result.cobble);

        let_cxx_string!(mossycobble = BASENODE_MOSSYCOBBLE);
        node_def_manager.getId(&mossycobble, &mut result.mossycobble);

        let_cxx_string!(apple = BASENODE_APPLE);
        node_def_manager.getId(&apple, &mut result.apple);

        let_cxx_string!(ice = BASENODE_ICE);
        node_def_manager.getId(&ice, &mut result.ice);

        let_cxx_string!(snow = BASENODE_SNOW);
        node_def_manager.getId(&snow, &mut result.snow);

        let_cxx_string!(snowblock = BASENODE_SNOWBLOCK);
        node_def_manager.getId(&snowblock, &mut result.snowblock);

        result
    }
}

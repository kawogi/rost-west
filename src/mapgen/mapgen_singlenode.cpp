// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013-2018 celeron55, Perttu Ahola <celeron55@gmail.com>
// Copyright (C) 2013-2018 kwolekr, Ryan Kwolek <kwolekr@minetest.net>
// Copyright (C) 2015-2018 paramat

#include "mapgen_singlenode.h"
#include "voxel.h"
#include "mapblock.h"
#include "mapnode.h"
#include "map.h"
#include "nodedef.h"
#include "voxelalgorithms.h"
#include "emerge.h"

#include "../rustlantis/rustlantis.h"

MapgenSinglenode::MapgenSinglenode(MapgenParams *params, EmergeParams *emerge)
	: Mapgen(MAPGEN_SINGLENODE, params, emerge)
{
	c_node = ndef->getId("mapgen_singlenode");
	if (c_node == CONTENT_IGNORE)
		c_node = CONTENT_AIR;

	MapNode n_node(c_node);
	set_light = (ndef->getLightingFlags(n_node).sunlight_propagates) ? LIGHT_SUN : 0x00;
	this->mapgen_id = rustlantis::mapgen_new();
}

MapgenSinglenode::~MapgenSinglenode()
{
	rustlantis::mapgen_destroy(this->mapgen_id);
}

//////////////////////// Map generator

void MapgenSinglenode::makeChunk(BlockMakeData *data)
{
	// Pre-conditions
	assert(data->vmanip);
	assert(data->nodedef);

	// MMVManip *vmanip = nullptr;
	// // Global map seed
	// u64 seed = 0;
	// v3s16 blockpos_min;
	// v3s16 blockpos_max;
	// UniqueQueue<v3s16> transforming_liquid;
	// const NodeDefManager *nodedef = nullptr;

	this->generating = true;
	this->vm = data->vmanip;
	this->ndef = data->nodedef;

	v3s16 blockpos_min = data->blockpos_min;
	v3s16 blockpos_max = data->blockpos_max;

	std::array<s16, 3> ffi_blockpos_min = {data->blockpos_min.X, data->blockpos_min.Y, data->blockpos_min.Z};
	std::array<s16, 3> ffi_blockpos_max = {data->blockpos_max.X, data->blockpos_max.Y, data->blockpos_max.Z};
	std::array<s32, 3> ffi_extent = vm->m_area.get_extent();
	// std::array<int, 3> a2 = {1, 2, 3};

	// Area of central chunk
	v3s16 node_min = blockpos_min * MAP_BLOCKSIZE;
	v3s16 node_max = (blockpos_max + v3s16(1, 1, 1)) * MAP_BLOCKSIZE - v3s16(1, 1, 1);

	blockseed = getBlockSeed2(node_min, data->seed);

	MapNode n_node(c_node);

	// auto size = blockpos_max - blockpos_min;
	auto block_count = (u32)(ffi_extent[0] * ffi_extent[1] * ffi_extent[2]); // u64(size.X) * u64(size.Y) * u64(size.Z);

	u32 *m_data = reinterpret_cast<u32 *>(vm->m_data);

	// u32 i = vm->m_area.index(node_min.X, y, z);
	rustlantis::mapgen_make_chunk(this->mapgen_id, ffi_blockpos_min, ffi_blockpos_max, ffi_extent, vm->m_area, *data->nodedef, ::rust::Slice(m_data, block_count));
	// rustlantis::make_chunk(::rust::Slice(m_data, block_count));

	// for (s16 z = node_min.Z; z <= node_max.Z; z++)
	// 	for (s16 y = node_min.Y; y <= node_max.Y; y++)
	// 	{
	// 		u32 i = vm->m_area.index(node_min.X, y, z);
	// 		for (s16 x = node_min.X; x <= node_max.X; x++)
	// 		{
	// 			if (vm->m_data[i].getContent() == CONTENT_IGNORE)
	// 				vm->m_data[i] = n_node;
	// 			i++;
	// 		}
	// 	}

	if (ndef->get(n_node).isLiquid())
		updateLiquid(&data->transforming_liquid, node_min, node_max);

	// Set lighting
	if ((flags & MG_LIGHT) && set_light == LIGHT_SUN)
		setLighting(LIGHT_SUN, node_min, node_max);

	this->generating = false;
}

int MapgenSinglenode::getSpawnLevelAtPoint(v2s16 p)
{
	return 0;
}

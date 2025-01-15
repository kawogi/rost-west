// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#include "version.h"
#include "config.h"

#if USE_CMAKE_CONFIG_H
	#include "cmake_config_githash.h"
#endif

#ifndef VERSION_GITHASH
	#define VERSION_GITHASH VERSION_STRING
#endif

#define STRINGIFY(x) #x
#define STR(x) STRINGIFY(x)

const char *g_version_string = VERSION_STRING;
const char *g_version_hash = VERSION_GITHASH;
const char *g_build_info =
	"BUILD_TYPE=" BUILD_TYPE "\n"
	"USE_CURL=" STR(USE_CURL) "\n"
	"STATIC_SHAREDIR=" STR(STATIC_SHAREDIR)
;

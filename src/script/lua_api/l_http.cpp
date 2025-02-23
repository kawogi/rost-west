// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#include "lua_api/l_internal.h"
#include "common/c_converter.h"
#include "common/c_content.h"
#include "lua_api/l_http.h"
#include "cpp_api/s_security.h"
#include "httpfetch.h"
#include "settings.h"
#include "debug.h"
#include "log.h"

#include <iomanip>

#define HTTP_API(name) \
	lua_pushstring(L, #name); \
	lua_pushcfunction(L, l_http_##name); \
	lua_settable(L, -3);

int ModApiHttp::l_set_http_api_lua(lua_State *L)
{
	NO_MAP_LOCK_REQUIRED;

	return 0;
}

void ModApiHttp::Initialize(lua_State *L, int top)
{
	// Define this function anyway so builtin can call it without checking
	API_FCT(set_http_api_lua);
}

void ModApiHttp::InitializeAsync(lua_State *L, int top)
{
}

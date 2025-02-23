// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#pragma once

#include "lua_api/l_base.h"
#include "config.h"

struct HTTPFetchRequest;
struct HTTPFetchResult;

class ModApiHttp : public ModApiBase {
private:
	// set_http_api_lua() [internal]
	static int l_set_http_api_lua(lua_State *L);


public:
	static void Initialize(lua_State *L, int top);
	static void InitializeAsync(lua_State *L, int top);
};

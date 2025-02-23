// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#include "httpfetch.h"
#include "porting.h" // for sleep_ms(), get_sysinfo(), secure_rand_fill_buf()
#include <list>
#include <unordered_map>
#include <cerrno>
#include <mutex>
#include "threading/event.h"
#include "config.h"
#include "exceptions.h"
#include "debug.h"
#include "log.h"
#include "porting.h"
#include "util/container.h"
#include "util/thread.h"
#include "version.h"
#include "settings.h"
#include "noise.h"

static std::mutex g_httpfetch_mutex;
static std::unordered_map<u64, std::queue<HTTPFetchResult>>
	g_httpfetch_results;
static PcgRandom g_callerid_randomness;

static std::string default_user_agent()
{
	std::string ret(PROJECT_NAME_C "/");
	ret.append(g_version_string).append(" (").append(porting::get_sysinfo()).append(")");
	return ret;
}

HTTPFetchRequest::HTTPFetchRequest() :
	timeout(g_settings->getS32("curl_timeout")),
	connect_timeout(10 * 1000),
	useragent(default_user_agent())
{
	timeout = std::max(timeout, MIN_HTTPFETCH_TIMEOUT_INTERACTIVE);
}

void httpfetch_caller_free(u64 caller)
{
	verbosestream<<"httpfetch_caller_free: freeing "
			<<caller<<std::endl;

	if (caller != HTTPFETCH_DISCARD) {
		MutexAutoLock lock(g_httpfetch_mutex);
		g_httpfetch_results.erase(caller);
	}
}

/*
	USE_CURL is off:

	Dummy httpfetch implementation that always returns an error.
*/

bool httpfetch_sync_interruptible(const HTTPFetchRequest &fetch_request,
		HTTPFetchResult &fetch_result, long interval)
{
	errorstream << "httpfetch_sync_interruptible: unable to fetch " << fetch_request.url
			<< " because USE_CURL=0" << std::endl;

	fetch_result = HTTPFetchResult(fetch_request); // sets succeeded = false etc.
	return false;
}

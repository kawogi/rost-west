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


static void httpfetch_deliver_result(const HTTPFetchResult &fetch_result)
{
	u64 caller = fetch_result.caller;
	if (caller != HTTPFETCH_DISCARD) {
		MutexAutoLock lock(g_httpfetch_mutex);
		g_httpfetch_results[caller].emplace(fetch_result);
	}
}

static void httpfetch_request_clear(u64 caller);

u64 httpfetch_caller_alloc()
{
	MutexAutoLock lock(g_httpfetch_mutex);

	// Check each caller ID except reserved ones
	for (u64 caller = HTTPFETCH_CID_START; caller != 0; ++caller) {
		auto it = g_httpfetch_results.find(caller);
		if (it == g_httpfetch_results.end()) {
			verbosestream << "httpfetch_caller_alloc: allocating "
					<< caller << std::endl;
			// Access element to create it
			g_httpfetch_results[caller];
			return caller;
		}
	}

	FATAL_ERROR("httpfetch_caller_alloc: ran out of caller IDs");
}

u64 httpfetch_caller_alloc_secure()
{
	MutexAutoLock lock(g_httpfetch_mutex);

	// Generate random caller IDs and make sure they're not
	// already used or reserved.
	// Give up after 100 tries to prevent infinite loop
	size_t tries = 100;
	u64 caller;

	do {
		caller = (((u64) g_callerid_randomness.next()) << 32) |
				g_callerid_randomness.next();

		if (--tries < 1) {
			FATAL_ERROR("httpfetch_caller_alloc_secure: ran out of caller IDs");
			return HTTPFETCH_DISCARD;
		}
	} while (caller >= HTTPFETCH_CID_START &&
		g_httpfetch_results.find(caller) != g_httpfetch_results.end());

	verbosestream << "httpfetch_caller_alloc_secure: allocating "
		<< caller << std::endl;

	// Access element to create it
	g_httpfetch_results[caller];
	return caller;
}

void httpfetch_caller_free(u64 caller)
{
	verbosestream<<"httpfetch_caller_free: freeing "
			<<caller<<std::endl;

	httpfetch_request_clear(caller);
	if (caller != HTTPFETCH_DISCARD) {
		MutexAutoLock lock(g_httpfetch_mutex);
		g_httpfetch_results.erase(caller);
	}
}

bool httpfetch_async_get(u64 caller, HTTPFetchResult &fetch_result)
{
	MutexAutoLock lock(g_httpfetch_mutex);

	// Check that caller exists
	auto it = g_httpfetch_results.find(caller);
	if (it == g_httpfetch_results.end())
		return false;

	// Check that result queue is nonempty
	std::queue<HTTPFetchResult> &caller_results = it->second;
	if (caller_results.empty())
		return false;

	// Pop first result
	fetch_result = std::move(caller_results.front());
	caller_results.pop();
	return true;
}

/*
	USE_CURL is off:

	Dummy httpfetch implementation that always returns an error.
*/

void httpfetch_init(int parallel_limit)
{
}

void httpfetch_cleanup()
{
}

void httpfetch_async(const HTTPFetchRequest &fetch_request)
{
	errorstream << "httpfetch_async: unable to fetch " << fetch_request.url
			<< " because USE_CURL=0" << std::endl;

	HTTPFetchResult fetch_result(fetch_request); // sets succeeded = false etc.
	httpfetch_deliver_result(fetch_result);
}

static void httpfetch_request_clear(u64 caller)
{
}

bool httpfetch_sync_interruptible(const HTTPFetchRequest &fetch_request,
		HTTPFetchResult &fetch_result, long interval)
{
	errorstream << "httpfetch_sync_interruptible: unable to fetch " << fetch_request.url
			<< " because USE_CURL=0" << std::endl;

	fetch_result = HTTPFetchResult(fetch_request); // sets succeeded = false etc.
	return false;
}

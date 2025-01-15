// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

/*
	Random portability stuff
*/

#pragma once

#if !defined(_GNU_SOURCE)
	#define _GNU_SOURCE
#endif

// Be mindful of what you include here!
#include <string>
#include "config.h"
#include "irrlichttypes.h" // u64
#include "debug.h"
#include "constants.h"
#include "util/timetaker.h" // TimePrecision

#define SWPRINTF_CHARSTRING L"%s"

#include <unistd.h>
#include <cstdlib> // setenv

#define SLEEP_ACCURACY_US 200

#define sleep_ms(x) usleep((x)*1000)
#define sleep_us(x) usleep(x)

#if !HAVE_STRLCPY
	#define strlcpy(d, s, n) mystrlcpy(d, s, n)
#endif

#include <sys/time.h>
#include <ctime>

namespace porting
{

/*
	Signal handler (grabs Ctrl-C on POSIX systems)
*/

void signal_handler_init();
// Returns a pointer to a bool.
// When the bool is true, program should quit.
bool * signal_handler_killstatus();

/*
	Path of static data directory.
*/
extern std::string path_share;

/*
	Directory for storing user data. Examples:
	Linux: "~/.<PROJECT_NAME>"
	Mac: "~/Library/Application Support/<PROJECT_NAME>"
*/
extern std::string path_user;

/*
	Path to directory for storing caches.
*/
extern std::string path_cache;

/*
	Gets the path of our executable.
*/
bool getCurrentExecPath(char *buf, size_t len);

/*
	Concatenate subpath to path_share.
*/
std::string getDataPath(const char *subpath);

/*
	Initialize path_*.
*/
void initializePaths();

/*
	Return system information
	e.g. "Linux/3.12.7 x86_64"
*/
const std::string &get_sysinfo();


// Monotonic timer
inline void os_get_clock(struct timespec *ts)
{
#if defined(CLOCK_MONOTONIC_RAW)
	clock_gettime(CLOCK_MONOTONIC_RAW, ts);
#elif defined(_POSIX_MONOTONIC_CLOCK) && _POSIX_MONOTONIC_CLOCK > 0
	clock_gettime(CLOCK_MONOTONIC, ts);
#else
# if defined(_POSIX_MONOTONIC_CLOCK) && _POSIX_MONOTONIC_CLOCK == 0
	// zero means it might be supported at runtime
	if (clock_gettime(CLOCK_MONOTONIC, ts) == 0)
		return;
# endif
	struct timeval tv;
	gettimeofday(&tv, NULL);
	TIMEVAL_TO_TIMESPEC(&tv, ts);
#endif
}

inline u64 getTimeS()
{
	struct timespec ts;
	os_get_clock(&ts);
	return ts.tv_sec;
}

inline u64 getTimeMs()
{
	struct timespec ts;
	os_get_clock(&ts);
	return ((u64) ts.tv_sec) * 1000LL + ((u64) ts.tv_nsec) / 1000000LL;
}

inline u64 getTimeUs()
{
	struct timespec ts;
	os_get_clock(&ts);
	return ((u64) ts.tv_sec) * 1000000LL + ((u64) ts.tv_nsec) / 1000LL;
}

inline u64 getTimeNs()
{
	struct timespec ts;
	os_get_clock(&ts);
	return ((u64) ts.tv_sec) * 1000000000LL + ((u64) ts.tv_nsec);
}

inline u64 getTime(TimePrecision prec)
{
	switch (prec) {
	case PRECISION_SECONDS: return getTimeS();
	case PRECISION_MILLI:   return getTimeMs();
	case PRECISION_MICRO:   return getTimeUs();
	case PRECISION_NANO:    return getTimeNs();
	}
	FATAL_ERROR("Called getTime with invalid time precision");
}

/**
 * Delta calculation function arguments.
 * @param old_time_ms old time for delta calculation
 * @param new_time_ms new time for delta calculation
 * @return positive delta value
 */
inline u64 getDeltaMs(u64 old_time_ms, u64 new_time_ms)
{
	if (new_time_ms >= old_time_ms) {
		return (new_time_ms - old_time_ms);
	}

	return (old_time_ms - new_time_ms);
}

inline void preciseSleepUs(u64 sleep_time)
{
	if (sleep_time > 0)
	{
		u64 target_time = porting::getTimeUs() + sleep_time;
		if (sleep_time > SLEEP_ACCURACY_US)
			sleep_us(sleep_time - SLEEP_ACCURACY_US);

		// Busy-wait the remaining time to adjust for sleep inaccuracies
		// The target - now > 0 construct will handle overflow gracefully (even though it should
		// never happen)
		while ((s64)(target_time - porting::getTimeUs()) > 0) {}
	}
}

inline const char *getPlatformName()
{
	return "Linux";
}

bool secure_rand_fill_buf(void *buf, size_t len);

// Call once near beginning of main function.
void osSpecificInit();

// This attaches to the parents process console, or creates a new one if it doesnt exist.
void attachOrCreateConsole();

#if HAVE_MALLOC_TRIM
/**
 * Call this after freeing bigger blocks of memory. Used on some platforms to
 * properly give memory back to the OS.
 * @param amount Number of bytes freed
*/
void TrackFreedMemory(size_t amount);

/**
 * Call this regularly from background threads. This performs the actual trimming
 * and is potentially slow.
 */
void TriggerMemoryTrim();
#else
static inline void TrackFreedMemory(size_t amount) { (void)amount; }
static inline void TriggerMemoryTrim() { (void)0; }
#endif

// snprintf wrapper
int mt_snprintf(char *buf, const size_t buf_size, const char *fmt, ...);

/**
 * Opens URL in default web browser
 *
 * Must begin with http:// or https://, and not contain any new lines
 *
 * @param url The URL
 * @return true on success, false on failure
 */
bool open_url(const std::string &url);

/**
 * Opens a directory in the default file manager
 *
 * The directory must exist.
 *
 * @param path Path to directory
 * @return true on success, false on failure
 */
bool open_directory(const std::string &path);

} // namespace porting

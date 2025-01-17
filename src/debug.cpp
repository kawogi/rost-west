// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>


#include "porting.h"
#include "debug.h"
#include "exceptions.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <sstream>
#include <thread>
#include "threading/mutex_auto_lock.h"
#include "config.h"

/*
	Assert
*/

void sanity_check_fn(const char *assertion, const char *file,
		unsigned int line, const char *function)
{

	errorstream << std::endl << "In thread " << std::hex
		<< std::this_thread::get_id() << ":\n" << std::dec;
	errorstream << file << ":" << line << ": " << function
		<< ": An engine assumption '" << assertion << "' failed." << std::endl;

	abort();
}

void fatal_error_fn(const char *msg, const char *file,
		unsigned int line, const char *function)
{
	errorstream << std::endl << "In thread " << std::hex
		<< std::this_thread::get_id() << ":\n" << std::dec;
	errorstream << file << ":" << line << ": " << function
		<< ": A fatal error occurred: " << msg << std::endl;

	abort();
}

std::string debug_describe_exc(const std::exception &e)
{
	if (dynamic_cast<const std::bad_alloc*>(&e))
		return "C++ out of memory";
	return std::string("\"").append(e.what()).append("\"");
}

void debug_set_exception_handler()
{
}


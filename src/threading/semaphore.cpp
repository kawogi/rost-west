// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 sapier <sapier AT gmx DOT net>

#include "threading/semaphore.h"

#include <iostream>
#include <cstdlib>
#include <cassert>

#define UNUSED(expr) do { (void)(expr); } while (0)

#include <cerrno>
#include <sys/time.h>
#include <pthread.h>


Semaphore::Semaphore(int val)
{
	int ret = sem_init(&semaphore, 0, val);
	assert(!ret);
	UNUSED(ret);
}


Semaphore::~Semaphore()
{
	int ret = sem_destroy(&semaphore);
	assert(!ret);
	UNUSED(ret);
}


void Semaphore::post(unsigned int num)
{
	assert(num > 0);
	for (unsigned i = 0; i < num; i++) {
		int ret = sem_post(&semaphore);
		assert(!ret);
		UNUSED(ret);
	}
}


void Semaphore::wait()
{
	int ret = sem_wait(&semaphore);
	assert(!ret);
	UNUSED(ret);
}


bool Semaphore::wait(unsigned int time_ms)
{
	int ret;
	if (time_ms > 0) {
		struct timespec wait_time;
		struct timeval now;

		if (gettimeofday(&now, NULL) == -1) {
			std::cerr << "Semaphore::wait(ms): Unable to get time with gettimeofday!" << std::endl;
			abort();
		}

		wait_time.tv_nsec = ((time_ms % 1000) * 1000 * 1000) + (now.tv_usec * 1000);
		wait_time.tv_sec  = (time_ms / 1000) + (wait_time.tv_nsec / (1000 * 1000 * 1000)) + now.tv_sec;
		wait_time.tv_nsec %= 1000 * 1000 * 1000;

		ret = sem_timedwait(&semaphore, &wait_time);
	} else {
		ret = sem_trywait(&semaphore);
	}

	assert(!ret || (errno == ETIMEDOUT || errno == EINTR || errno == EAGAIN));
	return !ret;

}


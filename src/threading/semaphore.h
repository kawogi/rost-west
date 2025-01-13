// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 sapier <sapier AT gmx DOT net>

#pragma once

#include <semaphore.h>

#include "util/basic_macros.h"

class Semaphore
{
public:
	Semaphore(int val = 0);
	~Semaphore();

	DISABLE_CLASS_COPY(Semaphore);

	void post(unsigned int num = 1);
	void wait();
	bool wait(unsigned int time_ms);

private:
	sem_t semaphore;
};

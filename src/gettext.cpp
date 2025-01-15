// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#include <string>
#include <cstring>
#include <iostream>
#include "gettext.h"
#include "util/string.h"
#include "porting.h"
#include "log.h"

/******************************************************************************/
void init_gettext()
{
	/* set current system default locale */
	setlocale(LC_ALL, "");

	/* no matter what locale is used we need number format to be "C" */
	/* to ensure formspec parameters are evaluated correctly!        */

	setlocale(LC_NUMERIC, "C");
	infostream << "Message locale is now set to: "
			<< setlocale(LC_ALL, 0) << std::endl;
}

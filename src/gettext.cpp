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
void init_gettext(const char *path, const std::string &configured_language,
	int argc, char *argv[])
{
#if USE_GETTEXT
	// First, try to set user override environment
	if (!configured_language.empty()) {
		// Set LANGUAGE which overrides all others, see
		// <https://www.gnu.org/software/gettext/manual/html_node/Locale-Environment-Variables.html>
		setenv("LANGUAGE", configured_language.c_str(), 1);

		// Reload locale with changed environment
		setlocale(LC_ALL, "");
	} else {
		/* set current system default locale */
		setlocale(LC_ALL, "");
	}

	std::string name = lowercase(PROJECT_NAME);
	infostream << "Gettext: domainname=\"" << name
		<< "\" path=\"" << path << "\"" << std::endl;

	bindtextdomain(name.c_str(), path);
	textdomain(name.c_str());

#else
	/* set current system default locale */
	setlocale(LC_ALL, "");
#endif // if USE_GETTEXT

	/* no matter what locale is used we need number format to be "C" */
	/* to ensure formspec parameters are evaluated correctly!        */

	setlocale(LC_NUMERIC, "C");
	infostream << "Message locale is now set to: "
			<< setlocale(LC_ALL, 0) << std::endl;
}

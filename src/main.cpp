// Luanti
// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2010-2013 celeron55, Perttu Ahola <celeron55@gmail.com>

#include "chat_interface.h"
#include "config.h"
#include "database/database.h"
#include "debug.h"
#include "defaultsettings.h"
#include "filesys.h"
#include "gameparams.h"
#include "gettext.h"
#include "httpfetch.h"
#include "irrlicht.h" // createDevice
#include "irrlicht_changes/printing.h"
#include "irrlichttypes_bloated.h"
#include "log.h"
#include "log_internal.h"
#include "mapblock.h"
#include "network/socket.h"
#include "player.h"
#include "porting.h"
#include "serialization.h" // SER_FMT_VER_HIGHEST_*
#include "server.h"
#include "version.h"

// for version information only
extern "C" {
#if USE_LUAJIT
#include <luajit.h>
#else
#include <lua.h>
#endif
}

// TODO: luanti.conf with migration
#define CONFIGFILE "minetest.conf"
#define DEBUGFILE "debug.txt"
#define DEFAULT_SERVER_PORT 30000

#define ENV_NO_COLOR "NO_COLOR"
#define ENV_CLICOLOR "CLICOLOR"
#define ENV_CLICOLOR_FORCE "CLICOLOR_FORCE"

typedef std::map<std::string, ValueSpec> OptionList;

/**********************************************************************
 * Private functions
 **********************************************************************/

static void set_allowed_options(OptionList *allowed_options);

static void print_worldspecs(const std::vector<WorldSpec> &worldspecs,
                             std::ostream &os, bool print_name = true,
                             bool print_path = true);

static void uninit_common();
static bool read_config_file(const Settings &cmd_args);

static bool game_configure_world(GameParams *game_params,
                                 const Settings &cmd_args);
static bool get_world_from_cmdline(GameParams *game_params,
                                   const Settings &cmd_args);
static bool get_world_from_config(GameParams *game_params,
                                  const Settings &cmd_args);
static bool auto_select_world(GameParams *game_params);
static std::string get_clean_world_path(const std::string &path);

static bool game_configure_subgame(GameParams *game_params,
                                   const Settings &cmd_args);
static bool get_game_from_cmdline(GameParams *game_params,
                                  const Settings &cmd_args);
static bool determine_subgame(GameParams *game_params);

/**********************************************************************/

static FileLogOutput file_log_output;

static OptionList allowed_options;

int main(int argc, char *argv[]) {
    g_logger.registerThread("Main");
    g_logger.addOutputMaxLevel(&stderr_output, LL_ACTION);
    g_logger.addOutput(&stderr_output, LL_INFO);

    Settings cmd_args;
    set_allowed_options(&allowed_options);
    if (!cmd_args.parseCommandLine(argc, argv, allowed_options)) {
        return 1;
    }

    // Debug handler
    BEGIN_DEBUG_EXCEPTION_HANDLER

    porting::signal_handler_init();
    porting::initializePaths();

    set_default_settings();

    sockets_init();

    // Initialize g_settings
    Settings::createLayer(SL_GLOBAL);

    // Set cleanup callback(s) to run at process exit
    atexit(uninit_common);

    if (!read_config_file(cmd_args)) {
        return 1;
    }

    // Initialize random seed
    {
        u32 seed = static_cast<u32>(time(nullptr)) << 16;
        seed |= porting::getTimeUs() & 0xffff;
        srand(seed);
        mysrand(seed);
    }

    // Initialize HTTP fetcher
    httpfetch_init(g_settings->getS32("curl_parallel_limit"));

    init_gettext();

    GameStartData game_params;

    if (cmd_args.exists("port")) {
        game_params.socket_port = cmd_args.getU16("port");
    } else {
        game_params.socket_port = g_settings->getU16("port");
    }

    if (game_params.socket_port == 0) {
        game_params.socket_port = DEFAULT_SERVER_PORT;
    }

    if (!game_configure_world(&game_params, cmd_args)) {
        errorstream << "No world path specified or found." << '\n';
        return 1;
    }

    game_configure_subgame(&game_params, cmd_args);

    sanity_check(!game_params.world_path.empty());

    verbosestream << _("Using world path") << " [" << game_params.world_path
                  << "]" << '\n';
    verbosestream << _("Using gameid") << " [" << game_params.game_spec.id
                  << "]" << '\n';

    // Bind address
    std::string bind_str = g_settings->get("bind_address");
    Address bind_addr(0, 0, 0, 0, game_params.socket_port);

    if (g_settings->getBool("ipv6_server")) {
        bind_addr.setAddress(static_cast<IPv6AddressBytes *>(nullptr));
    }
    try {
        bind_addr.Resolve(bind_str.c_str());
    } catch (const ResolveError &e) {
        warningstream << "Resolving bind address \"" << bind_str
                      << "\" failed: " << e.what()
                      << " -- Listening on all addresses." << '\n';
    }
    if (bind_addr.isIPv6() && !g_settings->getBool("enable_ipv6")) {
        errorstream << "Unable to listen on " << bind_addr.serializeString()
                    << " because IPv6 is disabled" << '\n';
        return 1;
    }

    try {
        // Create server
        Server server(game_params.world_path, game_params.game_spec, false,
                      bind_addr, true);
        server.start();

        // Run server
        bool &kill = *porting::signal_handler_killstatus();
        dedicated_server_loop(server, kill);
    } catch (const ModError &e) {
        errorstream << "ModError: " << e.what() << '\n';
        return 1;
    } catch (const ServerError &e) {
        errorstream << "ServerError: " << e.what() << '\n';
        return 1;
    }

    return 0;

    END_DEBUG_EXCEPTION_HANDLER
}

/*****************************************************************************
 * Startup / Init
 *****************************************************************************/

static void set_allowed_options(OptionList *allowed_options) {
    assert(allowed_options);
    allowed_options->clear();

#define SERVER_ONLY ""
#define LOCAL_GAME ""

    allowed_options->insert(std::make_pair(
        "config", ValueSpec(VALUETYPE_STRING,
                            _("Load configuration from specified file"))));
    allowed_options->insert(std::make_pair(
        "port", ValueSpec(VALUETYPE_STRING, _("Set network port (UDP)"))));
    allowed_options->insert(std::make_pair(
        "world", ValueSpec(VALUETYPE_STRING, _("Set world path" LOCAL_GAME))));
    allowed_options->insert(std::make_pair(
        "worldname",
        ValueSpec(VALUETYPE_STRING, _("Set world by name" LOCAL_GAME))));
    allowed_options->insert(std::make_pair(
        "gameid",
        ValueSpec(VALUETYPE_STRING,
                  _("Set gameid (\"--gameid list\" prints available ones)"))));

#undef SERVER_ONLY
#undef LOCAL_GAME
}

static void print_worldspecs(const std::vector<WorldSpec> &worldspecs,
                             std::ostream &os, bool print_name,
                             bool print_path) {
    for (const WorldSpec &worldspec : worldspecs) {
        const auto &name = worldspec.name;
        const auto &path = worldspec.path;
        if (print_name && print_path) {
            os << "\t" << name << "\t\t" << path << '\n';
        } else if (print_name) {
            os << "\t" << name << '\n';
        } else if (print_path) {
            os << "\t" << path << '\n';
        }
    }
}

namespace {
[[maybe_unused]] std::string findProgram(const char *name) {
    char *path_c = getenv("PATH");
    if (!path_c) {
        return "";
    }
    std::istringstream iss(path_c);
    std::string checkpath;
    while (!iss.eof()) {
        std::getline(iss, checkpath, PATH_DELIM[0]);
        if (!checkpath.empty() && checkpath.back() != DIR_DELIM_CHAR) {
            checkpath.push_back(DIR_DELIM_CHAR);
        }
        checkpath.append(name);
        if (fs::IsExecutable(checkpath)) {
            return checkpath;
        }
    }
    return "";
}

[[maybe_unused]] const char *debuggerNames[] = {"gdb", "lldb"};

template <class T> void getDebuggerArgs(T &out, int i) {
    if (i == 0) {
        for (auto s : {"-q", "--batch", "-iex", "set confirm off", "-ex", "run",
                       "-ex", "bt", "--args"}) {
            out.push_back(s);
        }
    } else if (i == 1) {
        for (auto s : {"-Q", "-b", "-o", "run", "-k", "bt\nq", "--"}) {
            out.push_back(s);
        }
    }
}
} // namespace

static void uninit_common() {
    httpfetch_cleanup();

    sockets_cleanup();

    // It'd actually be okay to leak these but we want to please valgrind...
    for (int i = 0; i < (int)SL_TOTAL_COUNT; i++) {
        delete Settings::getLayer((SettingsLayer)i);
    }
}

static bool read_config_file(const Settings &cmd_args) {
    // Path of configuration file in use
    sanity_check(g_settings_path.empty()); // Sanity check

    if (cmd_args.exists("config")) {
        bool r = g_settings->readConfigFile(cmd_args.get("config").c_str());
        if (!r) {
            errorstream << "Could not read configuration from \""
                        << cmd_args.get("config") << "\"" << '\n';
            return false;
        }
        g_settings_path = cmd_args.get("config");
    } else {
        std::vector<std::string> filenames;
        filenames.push_back(porting::path_user + DIR_DELIM + CONFIGFILE);
        // Legacy configuration file location
        filenames.push_back(porting::path_user + DIR_DELIM + ".." + DIR_DELIM +
                            CONFIGFILE);

        // Try also from a lower level (to aid having the same configuration
        // for many RUN_IN_PLACE installs)
        filenames.push_back(porting::path_user + DIR_DELIM + ".." + DIR_DELIM +
                            ".." + DIR_DELIM + CONFIGFILE);

        for (const std::string &filename : filenames) {
            bool r = g_settings->readConfigFile(filename.c_str());
            if (r) {
                g_settings_path = filename;
                break;
            }
        }

        // If no path found, use the first one (menu creates the file)
        if (g_settings_path.empty()) {
            g_settings_path = filenames[0];
        }
    }
    infostream << "Global configuration file: " << g_settings_path << '\n';

    return true;
}

static bool game_configure_world(GameParams *game_params,
                                 const Settings &cmd_args) {
    if (get_world_from_cmdline(game_params, cmd_args)) {
        return true;
    }

    if (get_world_from_config(game_params, cmd_args)) {
        return true;
    }

    return auto_select_world(game_params);
}

static bool get_world_from_cmdline(GameParams *game_params,
                                   const Settings &cmd_args) {
    std::string commanded_world;

    // World name
    std::string commanded_worldname;
    if (cmd_args.exists("worldname")) {
        commanded_worldname = cmd_args.get("worldname");
    }

    // If a world name was specified, convert it to a path
    if (!commanded_worldname.empty()) {
        // Get information about available worlds
        std::vector<WorldSpec> worldspecs = getAvailableWorlds();
        bool found = false;
        for (const WorldSpec &worldspec : worldspecs) {
            std::string name = worldspec.name;
            if (name == commanded_worldname) {
                dstream << "Using world specified by --worldname on the "
                           "command line"
                        << '\n';
                commanded_world = worldspec.path;
                found = true;
                break;
            }
        }
        if (!found) {
            dstream << "World '" << commanded_worldname
                    << "' not available. Available worlds:" << '\n';
            print_worldspecs(worldspecs, dstream);
            return false;
        }

        game_params->world_path = get_clean_world_path(commanded_world);
        return !commanded_world.empty();
    }

    if (cmd_args.exists("world")) {
        commanded_world = cmd_args.get("world");
    } else if (cmd_args.exists("map-dir")) {
        commanded_world = cmd_args.get("map-dir");
    } else if (cmd_args.exists("nonopt0")) { // First nameless argument
        commanded_world = cmd_args.get("nonopt0");
    }

    game_params->world_path = get_clean_world_path(commanded_world);
    return !commanded_world.empty();
}

static bool get_world_from_config(GameParams *game_params,
                                  const Settings &cmd_args) {
    // World directory
    std::string commanded_world;

    if (g_settings->exists("map-dir")) {
        commanded_world = g_settings->get("map-dir");
    }

    game_params->world_path = get_clean_world_path(commanded_world);

    return !commanded_world.empty();
}

static bool auto_select_world(GameParams *game_params) {
    // No world was specified; try to select it automatically
    // Get information about available worlds

    std::vector<WorldSpec> worldspecs = getAvailableWorlds();
    std::string world_path;

    // If there is only a single world, use it
    if (worldspecs.size() == 1) {
        world_path = worldspecs[0].path;
        dstream << "Automatically selecting world at [" << world_path << "]"
                << '\n';
        // If there are multiple worlds, list them
    } else if (worldspecs.size() > 1) {
        rawstream
            << "Multiple worlds are available.\n"
            << "Please select one using --worldname <name> or --world <path>"
            << '\n';
        print_worldspecs(worldspecs, rawstream);
        return false;
        // If there are no worlds, automatically create a new one
    } else {
        // This is the ultimate default world path
        world_path =
            porting::path_user + DIR_DELIM + "worlds" + DIR_DELIM + "world";
        infostream << "Using default world at [" << world_path << "]" << '\n';
    }

    assert(!world_path.empty()); // Post-condition
    game_params->world_path = world_path;
    return true;
}

static std::string get_clean_world_path(const std::string &path) {
    const std::string worldmt = "world.mt";
    std::string clean_path;

    if (path.size() > worldmt.size() &&
        path.substr(path.size() - worldmt.size()) == worldmt) {
        dstream << _("Supplied world.mt file - stripping it off.") << '\n';
        clean_path = path.substr(0, path.size() - worldmt.size());
    } else {
        clean_path = path;
    }
    return path;
}

static bool game_configure_subgame(GameParams *game_params,
                                   const Settings &cmd_args) {
    bool success;

    success = get_game_from_cmdline(game_params, cmd_args);
    if (!success) {
        success = determine_subgame(game_params);
    }

    return success;
}

static bool get_game_from_cmdline(GameParams *game_params,
                                  const Settings &cmd_args) {
    SubgameSpec commanded_gamespec;

    if (cmd_args.exists("gameid")) {
        std::string gameid = cmd_args.get("gameid");
        commanded_gamespec = findSubgame(gameid);
        if (!commanded_gamespec.isValid()) {
            errorstream << "Game \"" << gameid << "\" not found" << '\n';
            return false;
        }
        dstream << _("Using game specified by --gameid on the command line")
                << '\n';
        game_params->game_spec = commanded_gamespec;
        return true;
    }

    return false;
}

static bool determine_subgame(GameParams *game_params) {
    SubgameSpec gamespec;

    assert(!game_params->world_path.empty()); // Pre-condition

    // If world doesn't exist
    if (!game_params->world_path.empty() &&
        !getWorldExists(game_params->world_path)) {
        // Try to take gamespec from command line
        if (game_params->game_spec.isValid()) {
            gamespec = game_params->game_spec;
            infostream << "Using commanded gameid [" << gamespec.id << "]"
                       << '\n';
        } else {
            std::string contentdb_url = g_settings->get("contentdb_url");

            // If this is a dedicated server and no gamespec has been
            // specified, print a friendly error pointing to ContentDB.
            errorstream
                << "To run a " PROJECT_NAME_C
                   " server, you need to select a game using the "
                   "'--gameid' argument."
                << '\n'
                << "Check out " << contentdb_url
                << " for a selection of games to pick from and download."
                << '\n';

            return false;
        }
    } else { // World exists
        std::string world_gameid =
            getWorldGameId(game_params->world_path, false);
        // If commanded to use a gameid, do so
        if (game_params->game_spec.isValid()) {
            gamespec = game_params->game_spec;
            if (game_params->game_spec.id != world_gameid) {
                warningstream
                    << "Using commanded gameid [" << gamespec.id << "]"
                    << " instead of world gameid [" << world_gameid << "]"
                    << '\n';
            }
        } else {
            // If world contains an embedded game, use it;
            // Otherwise find world from local system.
            gamespec = findWorldSubgame(game_params->world_path);
            infostream << "Using world gameid [" << gamespec.id << "]" << '\n';
        }
    }

    if (!gamespec.isValid()) {
        errorstream << "Game [" << gamespec.id << "] could not be found."
                    << '\n';
        return false;
    }

    game_params->game_spec = gamespec;
    return true;
}

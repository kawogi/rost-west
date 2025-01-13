# List of available parameters

```text
cmake . -LH
```

```text
// Use a blacklist to avoid known broken locales
APPLY_LOCALE_BLACKLIST:BOOL=ON

// Build documentation
BUILD_DOCUMENTATION:BOOL=TRUE

// Fetch and build with the Tracy profiler client
BUILD_WITH_TRACY:BOOL=FALSE

// Choose the type of build. Options are: None;Release;Debug;SemiDebug;RelWithDebInfo;MinSizeRel.
CMAKE_BUILD_TYPE:STRING=Release

// Install path prefix, prepended onto install directories.
CMAKE_INSTALL_PREFIX:PATH=/usr/local

// Directory to install binaries into
CUSTOM_BINDIR:STRING=

// Directory to install documentation into
CUSTOM_DOCDIR:STRING=

// Directory to install example config file into
CUSTOM_EXAMPLE_CONF_DIR:STRING=

// Directory to install icons into
CUSTOM_ICONDIR:STRING=

// Directory to install l10n files into
CUSTOM_LOCALEDIR:STRING=

// Directory to install manpages into
CUSTOM_MANDIR:STRING=

// Directory to install data files into
CUSTOM_SHAREDIR:STRING=

// Directory to install .desktop files into
CUSTOM_XDG_APPS_DIR:STRING=

// Enable cURL support for fetching media
ENABLE_CURL:BOOL=ON

// Enable ncurses console
ENABLE_CURSES:BOOL=ON

// Use GetText for internationalization
ENABLE_GETTEXT:BOOL=ON

// Enable OpenGL ES 2+
ENABLE_GLES2:BOOL=OFF

// Use Link Time Optimization
ENABLE_LTO:BOOL=TRUE

// Enable LuaJIT support
ENABLE_LUAJIT:BOOL=ON

// Enable OpenGL
ENABLE_OPENGL:BOOL=ON

// Enable OpenGL 3+
ENABLE_OPENGL3:BOOL=ON

// Use OpenSSL's libcrypto for faster SHA implementations
ENABLE_OPENSSL:BOOL=ON

// Enable prometheus client support
ENABLE_PROMETHEUS:BOOL=OFF

// Enable sound
ENABLE_SOUND:BOOL=ON

// Enable SpatialIndex AreaStore backend
ENABLE_SPATIAL:BOOL=ON

// Use GMP from system
ENABLE_SYSTEM_GMP:BOOL=ON

// Enable using a system-wide JsonCpp
ENABLE_SYSTEM_JSONCPP:BOOL=ON

// Whether to enable update checks by default
ENABLE_UPDATE_CHECKER:BOOL=(;NOT;TRUE;)

// Git tag for fetching Tracy client. Match with your server (gui) version
FETCH_TRACY_GIT_TAG:STRING=master

// Path to Gettext msgfmt
GETTEXT_MSGFMT:FILEPATH=/usr/bin/msgfmt

// Install Development Test
INSTALL_DEVTEST:BOOL=FALSE

// Path to a file listing all headers to precompile
PRECOMPILED_HEADERS_PATH:FILEPATH=

// Precompile some headers (experimental; requires CMake 3.16 or later)
PRECOMPILE_HEADERS:BOOL=OFF

// Require LuaJIT support
REQUIRE_LUAJIT:BOOL=OFF

// Run directly in source directory structure
RUN_IN_PLACE:BOOL=FALSE

// The directory containing a CMake configuration file for SDL2.
SDL2_DIR:PATH=/usr/lib/x86_64-linux-gnu/cmake/SDL2

// Path to a file.
SPATIAL_INCLUDE_DIR:PATH=SPATIAL_INCLUDE_DIR-NOTFOUND

// Path to a library.
SPATIAL_LIBRARY:FILEPATH=SPATIAL_LIBRARY-NOTFOUND

// Use -pg flag for g++
USE_GPROF:BOOL=FALSE

// Use the SDL2 backend
USE_SDL2:BOOL=ON

// Stuff to append to version string
VERSION_EXTRA:STRING=

// Enable -Wall for Release build
WARN_ALL:BOOL=TRUE
```

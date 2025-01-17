# List of available parameters

```text
cmake . -LH
```

```text
// Choose the type of build. Options are: None;Release;Debug;SemiDebug;RelWithDebInfo;MinSizeRel.
CMAKE_BUILD_TYPE:STRING=Release

// Install path prefix, prepended onto install directories.
CMAKE_INSTALL_PREFIX:PATH=/usr/local

// Enable cURL support for fetching media
ENABLE_CURL:BOOL=ON

// Use Link Time Optimization
ENABLE_LTO:BOOL=TRUE

// Enable LuaJIT support
ENABLE_LUAJIT:BOOL=ON

// Use OpenSSL's libcrypto for faster SHA implementations
ENABLE_OPENSSL:BOOL=ON

// Use GMP from system
ENABLE_SYSTEM_GMP:BOOL=ON

// Enable using a system-wide JsonCpp
ENABLE_SYSTEM_JSONCPP:BOOL=ON

// Whether to enable update checks by default
ENABLE_UPDATE_CHECKER:BOOL=(;NOT;TRUE;)

// Install Development Test
INSTALL_DEVTEST:BOOL=FALSE

// Require LuaJIT support
REQUIRE_LUAJIT:BOOL=OFF

// The directory containing a CMake configuration file for SDL2.
SDL2_DIR:PATH=/usr/lib/x86_64-linux-gnu/cmake/SDL2

// Enable -Wall for Release build
WARN_ALL:BOOL=TRUE
```

# Compiling Luanti

- [Compiling on GNU/Linux](linux.md)

## CMake options

General options and their default values:

    BUILD_DOCUMENTATION=TRUE   - Build doxygen documentation
    CMAKE_BUILD_TYPE=Release   - Type of build (Release vs. Debug)
        Release                - Release build
        Debug                  - Debug build
        SemiDebug              - Partially optimized debug build
        RelWithDebInfo         - Release build with debug information
        MinSizeRel             - Release build with -Os passed to compiler to make executable as small as possible
    PRECOMPILE_HEADERS=FALSE   - Precompile some headers (experimental; requires CMake 3.16 or later)
    PRECOMPILED_HEADERS_PATH=  - Path to a file listing all headers to precompile (default points to src/precompiled_headers.txt)
    USE_SDL2=TRUE              - Build with SDL2; Enables IrrlichtMt device SDL2
    ENABLE_CURL=ON             - Build with cURL; Enables use of online mod repo, public serverlist and remote media fetching via http
    ENABLE_CURSES=ON           - Build with (n)curses; Enables a server side terminal (command line option: --terminal)
    ENABLE_SPATIAL=ON          - Build with LibSpatial; Speeds up AreaStores
    ENABLE_OPENSSL=ON          - Build with OpenSSL; Speeds up SHA1 and SHA2 hashing
    ENABLE_LTO=<varies>        - Build with IPO/LTO optimizations (smaller and more efficient than regular build)
    ENABLE_LUAJIT=ON           - Build with LuaJIT (much faster than non-JIT Lua)
    ENABLE_PROMETHEUS=OFF      - Build with Prometheus metrics exporter (listens on tcp/30000 by default)
    ENABLE_SYSTEM_GMP=ON       - Use GMP from system (much faster than bundled mini-gmp)
    ENABLE_SYSTEM_JSONCPP=ON   - Use JsonCPP from system
    ENABLE_UPDATE_CHECKER=TRUE - Whether to enable update checks by default
    INSTALL_DEVTEST=FALSE      - Whether the Development Test game should be installed alongside Luanti
    USE_GPROF=FALSE            - Enable profiling using GProf
    BUILD_WITH_TRACY=FALSE     - Fetch and build with the Tracy profiler client
    FETCH_TRACY_GIT_TAG=master - Git tag for fetching Tracy client. Match with your server (gui) version
    VERSION_EXTRA=             - Text to append to version (e.g. VERSION_EXTRA=foobar -> Luanti 5.10.0-foobar)

Library specific options:

    SDL2_INCLUDE_DIRS               - Only if building with SDL2; directory where SDL.h is located
    SDL2_LIBRARIES                  - Only if building with SDL2; path to libSDL2.a/libSDL2.so/libSDL2.lib
    CURL_INCLUDE_DIR                - Only if building with cURL; directory where curl.h is located
    CURL_LIBRARY                    - Only if building with cURL; path to libcurl.a/libcurl.so/libcurl.lib
    FREETYPE_INCLUDE_DIR_freetype2  - Directory that contains files such as ftimage.h
    FREETYPE_INCLUDE_DIR_ft2build   - Directory that contains ft2build.h
    FREETYPE_LIBRARY                - Path to libfreetype.a/libfreetype.so/freetype.lib
    GMP_INCLUDE_DIR                 - Directory that contains gmp.h
    GMP_LIBRARY                     - Path to libgmp.a/libgmp.so/libgmp.lib
    ICONV_LIBRARY                   - Optional/platform-dependent; path to libiconv.so/libiconv.dylib
    JPEG_INCLUDE_DIR                - Directory that contains jpeg.h
    JPEG_LIBRARY                    - Path to libjpeg.a/libjpeg.so/libjpeg.lib
    JSON_INCLUDE_DIR                - Directory that contains json/allocator.h
    JSON_LIBRARY                    - Path to libjson.a/libjson.so/libjson.lib
    SPATIAL_INCLUDE_DIR             - Only when building with LibSpatial; directory that contains spatialindex/SpatialIndex.h
    SPATIAL_LIBRARY                 - Only when building with LibSpatial; path to libspatialindex.so/spatialindex-32.lib
    LUA_INCLUDE_DIR                 - Only if you want to use LuaJIT; directory where luajit.h is located
    LUA_LIBRARY                     - Only if you want to use LuaJIT; path to libluajit.a/libluajit.so
    OGG_INCLUDE_DIR                 - Only if building with sound; directory that contains an ogg directory which contains ogg.h
    OGG_LIBRARY                     - Only if building with sound; path to libogg.a/libogg.so/libogg.dll.a
    OPENAL_INCLUDE_DIR              - Only if building with sound; directory where al.h is located
    OPENAL_LIBRARY                  - Only if building with sound; path to libopenal.a/libopenal.so/OpenAL32.lib
    PNG_INCLUDE_DIR                 - Directory that contains png.h
    PNG_LIBRARY                     - Path to libpng.a/libpng.so/libpng.lib
    PROMETHEUS_PULL_LIBRARY         - Only if building with prometheus; path to prometheus-cpp-pull
    PROMETHEUS_CORE_LIBRARY         - Only if building with prometheus; path to prometheus-cpp-core
    PROMETHEUS_CPP_INCLUDE_DIR      - Only if building with prometheus; directory where prometheus/counter.h is located
    SQLITE3_INCLUDE_DIR             - Directory that contains sqlite3.h
    SQLITE3_LIBRARY                 - Path to libsqlite3.a/libsqlite3.so/sqlite3.lib
    VORBISFILE_LIBRARY              - Only if building with sound; path to libvorbisfile.a/libvorbisfile.so/libvorbisfile.dll.a
    VORBIS_INCLUDE_DIR              - Only if building with sound; directory that contains a directory vorbis with vorbisenc.h inside
    VORBIS_LIBRARY                  - Only if building with sound; path to libvorbis.a/libvorbis.so/libvorbis.dll.a
    ZLIB_INCLUDE_DIR                - Directory that contains zlib.h
    ZLIB_LIBRARY                    - Path to libz.a/libz.so/zlib.lib
    ZSTD_INCLUDE_DIR                - Directory that contains zstd.h
    ZSTD_LIBRARY                    - Path to libzstd.a/libzstd.so/ztd.lib

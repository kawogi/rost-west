# Compiling Luanti

- [Compiling on GNU/Linux](linux.md)

## CMake options

General options and their default values:

    CMAKE_BUILD_TYPE=Release   - Type of build (Release vs. Debug)
        Release                - Release build
        Debug                  - Debug build
        SemiDebug              - Partially optimized debug build
        RelWithDebInfo         - Release build with debug information
        MinSizeRel             - Release build with -Os passed to compiler to make executable as small as possible
    ENABLE_OPENSSL=ON          - Build with OpenSSL; Speeds up SHA1 and SHA2 hashing
    ENABLE_LTO=<varies>        - Build with IPO/LTO optimizations (smaller and more efficient than regular build)
    ENABLE_LUAJIT=ON           - Build with LuaJIT (much faster than non-JIT Lua)
    ENABLE_PROMETHEUS=OFF      - Build with Prometheus metrics exporter (listens on tcp/30000 by default)
    ENABLE_SYSTEM_GMP=ON       - Use GMP from system (much faster than bundled mini-gmp)
    ENABLE_SYSTEM_JSONCPP=ON   - Use JsonCPP from system
    ENABLE_UPDATE_CHECKER=TRUE - Whether to enable update checks by default
    INSTALL_DEVTEST=FALSE      - Whether the Development Test game should be installed alongside Luanti

Library specific options:

    SDL2_INCLUDE_DIRS               - Only if building with SDL2; directory where SDL.h is located
    SDL2_LIBRARIES                  - Only if building with SDL2; path to libSDL2.a/libSDL2.so/libSDL2.lib
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

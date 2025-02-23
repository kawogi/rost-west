# Compiling on GNU/Linux

## Dependencies

| Dependency | Version | Commentary |
| ---------- | ------- | ---------- |
| GCC        | 7.5+    | or Clang 7.0.1+ |
| CMake      | 3.5+    |            |
| libjpeg    | -       |            |
| libpng     | -       |            |
| SDL        | 2.x     |            |
| Freetype   | 2.0+    |            |
| SQLite3    | 3+      |            |
| Zlib       | -       |            |
| Zstd       | 1.0+    |            |
| LuaJIT     | 2.0+    | Bundled Lua 5.1 is used if not present |
| GMP        | 5.0.0+  | Bundled mini-GMP is used if not present |
| JsonCPP    | 1.0.0+  | Bundled JsonCPP is used if not present |
| OpenSSL    | 3.0+    | Optional (only libcrypto used) |

For Debian/Ubuntu users:

    sudo apt install g++ make libc6-dev cmake libpng-dev libjpeg-dev libgl1-mesa-dev libsqlite3-dev libogg-dev libopenal-dev libfreetype6-dev zlib1g-dev libgmp-dev libjsoncpp-dev libzstd-dev libluajit-5.1-dev libsdl2-dev

For Fedora users:

    sudo dnf install make automake gcc gcc-c++ kernel-devel cmake openal-soft-devel libpng-devel libjpeg-devel libogg-devel freetype-devel mesa-libGL-devel zlib-devel jsoncpp-devel gmp-devel sqlite-devel luajit-devel spatialindex-devel libzstd-devel SDL2-devel

For openSUSE users:

 sudo zypper install gcc gcc-c++ cmake libjpeg8-devel libpng16-devel openal-soft-devel sqlite3-devel luajit-devel libzstd-devel Mesa-libGL-devel freetype2-devel SDL2-devel

For Arch users:

    sudo pacman -S --needed base-devel cmake libpng libjpeg-turbo sqlite libogg openal freetype2 jsoncpp gmp luajit zstd sdl2

For Alpine users:

    sudo apk add build-base cmake libpng-dev jpeg-dev mesa-dev sqlite-dev libogg-dev openal-soft-dev freetype-dev zlib-dev gmp-dev jsoncpp-dev luajit-dev zstd-dev sdl2-dev

For Void users:

    sudo xbps-install cmake libpng-devel jpeg-devel mesa sqlite-devel libogg-devel libopenal-devel freetype-devel zlib-devel gmp-devel jsoncpp-devel LuaJIT-devel zstd libzstd-devel SDL2-devel

## Download

You can install Git for easily keeping your copy up to date.
If you don’t want Git, read below on how to get the source without Git.
This is an example for installing Git on Debian/Ubuntu:

    sudo apt install git

For Fedora users:

    sudo dnf install git

For Arch users:

 sudo pacman -S git

For Alpine users:

 sudo apk add git

For Void users:

    sudo xbps-install git

Download source (this is the URL to the latest of source repository, which might not work at all times) using Git:

    git clone --depth 1 https://github.com/minetest/minetest.git
    cd minetest

Download source, without using Git:

    wget https://github.com/minetest/minetest/archive/master.tar.gz
    tar xf master.tar.gz
    cd minetest-master

## Build

Build a version that runs directly from the source directory:

    cmake .
    make -j$(nproc)

Run it:

    ./bin/minetest

- Use `cmake . -LH` to see all CMake options and their current state.
- You can select between Release and Debug build by `-DCMAKE_BUILD_TYPE=<Debug or Release>`.
  - Debug build is slower, but gives much more useful output in a debugger.

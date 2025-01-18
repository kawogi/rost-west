project(luanti)

INCLUDE(CheckTypeSize)
INCLUDE(CheckIncludeFiles)
INCLUDE(CheckLibraryExists)

check_type_size(int SIZEOF_INT BUILTIN_TYPES_ONLY LANGUAGE CXX)
if(SIZEOF_INT LESS 4)
	message(FATAL_ERROR "${PROJECT_NAME_CAPITALIZED} will not work with int less than 32 bits wide.")
endif()

check_type_size(size_t SIZEOF_SIZE_T LANGUAGE CXX)
if(SIZEOF_SIZE_T LESS 4)
	message(FATAL_ERROR "${PROJECT_NAME_CAPITALIZED} will not work with size_t less than 32 bits wide.")
endif()

# Add custom SemiDebug build mode
set(CMAKE_CXX_FLAGS_SEMIDEBUG "-O1 -g -Wall" CACHE STRING
	"Flags used by the C++ compiler during semidebug builds."
	FORCE
)
set(CMAKE_C_FLAGS_SEMIDEBUG "-O1 -g -Wall -pedantic" CACHE STRING
	"Flags used by the C compiler during semidebug builds."
	FORCE
)
mark_as_advanced(
	CMAKE_CXX_FLAGS_SEMIDEBUG
	CMAKE_C_FLAGS_SEMIDEBUG
)
set(SUPPORTED_BUILD_TYPES None Release Debug SemiDebug RelWithDebInfo MinSizeRel)
set(CMAKE_BUILD_TYPE "${CMAKE_BUILD_TYPE}" CACHE STRING
	"Choose the type of build. Options are: ${SUPPORTED_BUILD_TYPES}."
	FORCE
)
if(NOT (CMAKE_BUILD_TYPE IN_LIST SUPPORTED_BUILD_TYPES))
	message(WARNING
			"Unknown CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}. Options are: ${SUPPORTED_BUILD_TYPES}.")
endif()


# Set some random things default to not being visible in the GUI
mark_as_advanced(EXECUTABLE_OUTPUT_PATH LIBRARY_OUTPUT_PATH)

option(ENABLE_CURL "Enable cURL support for fetching media" TRUE)
set(USE_CURL FALSE)

if(ENABLE_CURL)
	find_package(CURL 7.28.0)
	if (CURL_FOUND)
		message(STATUS "cURL support enabled.")
		set(USE_CURL TRUE)
	endif()
else()
	mark_as_advanced(CLEAR CURL_LIBRARY CURL_INCLUDE_DIR)
endif()

if(NOT USE_CURL)
	message(WARNING "cURL is required to announce to the server list")
endif()

mark_as_advanced(GETTEXT_INCLUDE_DIR GETTEXT_LIBRARY GETTEXT_MSGFMT)
message(STATUS "GetText disabled.")

find_package(SQLite3 REQUIRED)

option(ENABLE_OPENSSL "Use OpenSSL's libcrypto for faster SHA implementations" TRUE)
set(USE_OPENSSL FALSE)

if(ENABLE_OPENSSL)
	find_package(OpenSSL 3.0)
	if(OPENSSL_FOUND)
		set(USE_OPENSSL TRUE)
		message(STATUS "OpenSSL's libcrypto SHA enabled.")
	else()
		message(STATUS "OpenSSL not found!")
	endif()
endif(ENABLE_OPENSSL)


find_package(ZLIB REQUIRED)
find_package(Zstd REQUIRED)


# Use cmake_config.h
add_compile_definitions(USE_CMAKE_CONFIG_H)

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)
set(PLATFORM_LIBS Threads::Threads)

# Unix probably
list(APPEND PLATFORM_LIBS ${CMAKE_DL_LIBS})

find_library(ICONV_LIBRARY iconv)
mark_as_advanced(ICONV_LIBRARY)
if (ICONV_LIBRARY)
	list(APPEND PLATFORM_LIBS ${ICONV_LIBRARY})
endif()

# On clang and gcc, some functionalities of std::atomic require -latomic.
# See <https://en.cppreference.com/w/cpp/atomic/atomic#Notes>.
# Note that find_library does not reliably find it so we have to resort to this.
# Also, passing -latomic is not always the same as adding atomic to the library list.
include(CheckCSourceCompiles)
if(NOT CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
	set(CMAKE_REQUIRED_LIBRARIES "-latomic")
	check_c_source_compiles("int main(){}" HAVE_LINK_ATOMIC)
	set(CMAKE_REQUIRED_LIBRARIES "")
	if(HAVE_LINK_ATOMIC)
		list(APPEND PLATFORM_LIBS "-latomic")
	endif()
endif()

include(CheckSymbolExists)
check_symbol_exists(strlcpy "string.h" HAVE_STRLCPY)
check_symbol_exists(malloc_trim "malloc.h" HAVE_MALLOC_TRIM)

check_include_files(endian.h HAVE_ENDIAN_H)

configure_file(
	"${PROJECT_SOURCE_DIR}/cmake_config.h.in"
	"${PROJECT_BINARY_DIR}/cmake_config.h"
)


# Add a target that always rebuilds cmake_config_githash.h
add_custom_target(GenerateVersion
	COMMAND ${CMAKE_COMMAND}
	-D "GENERATE_VERSION_SOURCE_DIR=${CMAKE_CURRENT_SOURCE_DIR}"
	-D "GENERATE_VERSION_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}"
	-D "VERSION_STRING=${VERSION_STRING}"
	-D "DEVELOPMENT_BUILD=${DEVELOPMENT_BUILD}"
	-P "${CMAKE_SOURCE_DIR}/cmake/Modules/GenerateVersion.cmake"
	WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")


add_subdirectory(threading)
add_subdirectory(content)
add_subdirectory(database)
add_subdirectory(mapgen)
add_subdirectory(network)
add_subdirectory(script)
add_subdirectory(util)
add_subdirectory(server)

# Source files that are identical between server & client builds.
# This means they don't use or include anything that depends on the
# CHECK_CLIENT_BUILD() macro. If you wrongly add something here there will be
# a compiler error and you need to instead add it to client_SRCS or common_SRCS.
set(independent_SRCS
	chat.cpp
	content_nodemeta.cpp
	convert_json.cpp
	craftdef.cpp
	debug.cpp
	face_position_cache.cpp
	gettext_plural_form.cpp
	httpfetch.cpp
	hud.cpp
	inventory.cpp
	itemstackmetadata.cpp
	log.cpp
	metadata.cpp
	modchannels.cpp
	nameidmapping.cpp
	nodemetadata.cpp
	nodetimer.cpp
	noise.cpp
	objdef.cpp
	object_properties.cpp
	particles.cpp
	profiler.cpp
	serialization.cpp
	settings.cpp
	staticobject.cpp
	terminal_chat_console.cpp
	texture_override.cpp
	tileanimation.cpp
	tool.cpp
	${common_network_SRCS}
	${content_SRCS}
	${database_SRCS}
	${threading_SRCS}
	${util_SRCS}
)

# /!\ Consider carefully before adding files here /!\
set(common_SRCS
	clientdynamicinfo.cpp
	collision.cpp
	content_mapnode.cpp
	defaultsettings.cpp
	emerge.cpp
	environment.cpp
	filesys.cpp
	gettext.cpp
	inventorymanager.cpp
	itemdef.cpp
	light.cpp
	main.cpp
	map_settings_manager.cpp
	map.cpp
	mapblock.cpp
	mapnode.cpp
	mapsector.cpp
	nodedef.cpp
	pathfinder.cpp
	player.cpp
	porting.cpp
	raycast.cpp
	reflowscan.cpp
	remoteplayer.cpp
	rollback_interface.cpp
	server.cpp
	serverenvironment.cpp
	servermap.cpp
	translation.cpp
	version.cpp
	voxel.cpp
	voxelalgorithms.cpp
	${common_SCRIPT_SRCS}
	${common_server_SRCS}
	${mapgen_SRCS}
	${server_network_SRCS}
)

# Server sources
# (nothing here because a client always comes with a server)
set(server_SRCS
	${common_SRCS}
)

# Avoid source_group on broken CMake version.
# see issue #7074 #7075
if (CMAKE_VERSION VERSION_GREATER 3.8.1)
	source_group(TREE ${PROJECT_SOURCE_DIR} PREFIX "Source Files" FILES ${server_SRCS})
endif()

include_directories(
	${PROJECT_BINARY_DIR}
	${PROJECT_SOURCE_DIR}
	${PROJECT_SOURCE_DIR}/script
)
include_directories(SYSTEM
	${ZLIB_INCLUDE_DIR}
	${ZSTD_INCLUDE_DIR}
	${SQLITE3_INCLUDE_DIR}
	${LUA_INCLUDE_DIR}
	${GMP_INCLUDE_DIR}
	${JSON_INCLUDE_DIR}
	${LUA_BIT_INCLUDE_DIR}
	# on Android, Luanti depends on SDL2 directly
	# on other platforms, only IrrlichtMt depends on SDL2
	"$<$<PLATFORM_ID:Android>:${SDL2_INCLUDE_DIRS}>"
)

if(USE_CURL)
	include_directories(${CURL_INCLUDE_DIR})
endif()


# When cross-compiling assume the user doesn't want to run the executable anyway,
# otherwise place it in <source dir>/bin/ since Luanti can only run from there.
if(NOT CMAKE_CROSSCOMPILING)
	set(EXECUTABLE_OUTPUT_PATH "${CMAKE_SOURCE_DIR}/bin")
endif()

# shared object target
add_library(EngineCommon OBJECT
	${independent_SRCS}
)
add_dependencies(EngineCommon GenerateVersion)
target_link_libraries(EngineCommon
	sha256
)
if(USE_OPENSSL)
	target_link_libraries(EngineCommon OpenSSL::Crypto)
endif()
get_target_property(
	IRRLICHT_INCLUDES IrrlichtMt::IrrlichtMt INTERFACE_INCLUDE_DIRECTORIES)
target_include_directories(EngineCommon PRIVATE ${IRRLICHT_INCLUDES})

# server target
add_executable(${PROJECT_NAME}server)
list(SORT server_SRCS)
target_sources(${PROJECT_NAME}server PRIVATE
$<TARGET_OBJECTS:EngineCommon>
${server_SRCS}
)
target_link_libraries(${PROJECT_NAME}server rustlantis)

add_subdirectory(rustlantis)
# include_directories(${CMAKE_BINARY_DIR}/rustlantis)
include_directories(rustlantis)


# don't link the irrlicht library
get_target_property(
	IRRLICHT_INCLUDES IrrlichtMt::IrrlichtMt INTERFACE_INCLUDE_DIRECTORIES)
target_include_directories(${PROJECT_NAME}server PRIVATE ${IRRLICHT_INCLUDES})
target_link_libraries(
	${PROJECT_NAME}server
	${ZLIB_LIBRARIES}
	${ZSTD_LIBRARY}
	${SQLITE3_LIBRARY}
	${JSON_LIBRARY}
	${LUA_LIBRARY}
	${LUA_BIT_LIBRARY}
	sha256
	${GMP_LIBRARY}
	${PLATFORM_LIBS}
)
target_compile_definitions(${PROJECT_NAME}server PRIVATE "MT_BUILDTARGET=2")
if(NOT USE_LUAJIT)
	set_target_properties(${PROJECT_NAME}server PROPERTIES
		# This is necessary for dynamic Lua modules
		# to work when Lua is statically linked (issue #10806)
		ENABLE_EXPORTS 1
	)
endif()

if (USE_OPENSSL)
	target_link_libraries(${PROJECT_NAME}server OpenSSL::Crypto)
endif()
if(USE_CURL)
	target_link_libraries(
		${PROJECT_NAME}server
		${CURL_LIBRARY}
	)
endif()

# See issue #4638
FILE(READ "${CMAKE_SOURCE_DIR}/src/unsupported_language_list.txt" GETTEXT_BLACKLISTED_LOCALES)
STRING(REGEX REPLACE "\n" ";" GETTEXT_BLACKLISTED_LOCALES "${GETTEXT_BLACKLISTED_LOCALES}")

# Set some optimizations and tweaks

include(CheckCSourceCompiles)

set(CMAKE_REQUIRED_INCLUDES ${LUA_INCLUDE_DIR})
if(USE_LUAJIT)
	# libm usually required if statically linking
	set(CMAKE_REQUIRED_LIBRARIES ${LUA_LIBRARY} m)
	# LuaJIT provides exactly zero ways to determine how recent it is (the version
	# is unchanged since 2017), however it happens that string buffers were added
	# after the changes which we care about so that works as an indicator.
	# (https://github.com/LuaJIT/LuaJIT/commit/4c6b669 March 2021)
	# Note: This is no longer true as of August 2023, but we're keeping the old check.
	unset(HAVE_RECENT_LJ CACHE)
	check_symbol_exists(luaopen_string_buffer "lualib.h" HAVE_RECENT_LJ)
	if(NOT HAVE_RECENT_LJ)
		string(CONCAT explanation_msg
			"You are using a relatively old version of LuaJIT. We recommend "
			"running a recent version (from git) as older ones are known not "
			"to build/work correctly in all cases.\n"
			"THIS APPLIES ESPECIALLY ON Linux/aarch64!")
		message(WARNING ${explanation_msg})
	endif()
elseif(NOT MSVC)
	set(CMAKE_REQUIRED_LIBRARIES "")
	unset(HAVE_ATCCALL CACHE)
	# Note: we need to check the function without having the library
	#       available for linking, so check_symbol_exists won't work.
	# Incidentally this doesn't seem to work on MSVC...
	check_c_source_compiles("#include <lua.h>\nint main(){return sizeof(lua_atccall);}" HAVE_ATCCALL)
	if(NOT HAVE_ATCCALL)
		string(CONCAT explanation_msg
			"It looks like you're trying to build ${PROJECT_NAME_CAPITALIZED} using a system-wide "
			"Lua installation. This is no longer supported because PUC Lua "
			"cannot interoperate with C++ correctly. Read src/unittest/test_lua.cpp "
			" for technical details.")
		message(FATAL_ERROR ${explanation_msg})
	endif()
endif()

if(MSVC)
	# Visual Studio
	add_compile_definitions(_WIN32_WINNT=0x0601 WIN32_LEAN_AND_MEAN _CRT_SECURE_NO_WARNINGS)
	# EHa enables SEH exceptions (used for catching segfaults)
	set(CMAKE_CXX_FLAGS_RELEASE "/EHa /Ox /MD /GS- /Zi /fp:fast /D NDEBUG /D _HAS_ITERATOR_DEBUGGING=0")
	if(CMAKE_SIZEOF_VOID_P EQUAL 4)
		set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /arch:SSE")
	endif()

	set(CMAKE_EXE_LINKER_FLAGS_RELEASE "/INCREMENTAL:NO /DEBUG /OPT:REF /OPT:ICF /SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup")

	set(CMAKE_CXX_FLAGS_SEMIDEBUG "/MDd /Zi /Ob0 /O1 /RTC1")

	# Debug build doesn't catch exceptions by itself
	# Add some optimizations because otherwise it's VERY slow
	set(CMAKE_CXX_FLAGS_DEBUG "/MDd /Zi /Ob0 /Od /RTC1")

	# Flags for C files (sqlite)
	# /MD = dynamically link to MSVCRxxx.dll
	set(CMAKE_C_FLAGS_RELEASE "/O2 /Ob2 /MD")

	# Flags that cannot be shared between cl and clang-cl
	# https://clang.llvm.org/docs/UsersManual.html#clang-cl
	if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
		add_compile_options(-fuse-ld=lld)

		# Disable pragma-pack warning
		set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -Wno-pragma-pack")
	else()
		add_compile_options(/MP)
		set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /TP /FD /GL")
		set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS_RELEASE} /LTCG")
	endif()
else()
	# GCC or compatible compilers such as Clang
	set(WARNING_FLAGS "-Wall -Wextra -Wno-unused-parameter -Werror=vla")
	if(WARN_ALL)
		set(RELEASE_WARNING_FLAGS "${WARNING_FLAGS}")
	else()
		set(RELEASE_WARNING_FLAGS "")
	endif()

	# Use a safe subset of flags to speed up math calculations:
	# - we don't need errno or math exceptions
	# - we don't deal with signed zero
	set(MATH_FLAGS "-fno-math-errno -fno-trapping-math -fno-signed-zeros")

	# Enable SSE for floating point math on 32-bit x86 by default
	# reasoning see minetest issue #11810 and https://gcc.gnu.org/wiki/FloatingPointMath
	if(CMAKE_SIZEOF_VOID_P EQUAL 4)
		check_c_source_compiles("#ifndef __i686__\n#error\n#endif\nint main(){}" IS_I686)
		if(IS_I686)
			message(STATUS "Detected Intel x86: using SSE instead of x87 FPU")
			set(OTHER_FLAGS "${OTHER_FLAGS} -mfpmath=sse -msse")
		endif()
	endif()

	set(CMAKE_CXX_FLAGS_RELEASE "-DNDEBUG ${RELEASE_WARNING_FLAGS} ${OTHER_FLAGS} -pipe -funroll-loops -O3 -fomit-frame-pointer")
	if(CMAKE_SYSTEM_NAME STREQUAL "Linux"
			AND CMAKE_CXX_COMPILER_ID STREQUAL "Clang"
			AND CMAKE_CXX_COMPILER_VERSION MATCHES "^9\\.")
		# Clang 9 has broken -ffast-math on glibc
	else()
		set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} ${MATH_FLAGS}")
	endif()
	set(CMAKE_CXX_FLAGS_RELWITHDEBINFO "${CMAKE_CXX_FLAGS_RELEASE} -g")
	set(CMAKE_CXX_FLAGS_SEMIDEBUG "-g -O1 ${WARNING_FLAGS} ${OTHER_FLAGS}")
	set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 ${WARNING_FLAGS} ${OTHER_FLAGS}")

	# enable assertions for libstdc++ or libc++
	set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -Wp,-D_GLIBCXX_ASSERTIONS -Wp,-D_LIBCPP_ENABLE_ASSERTIONS=1")
endif()

macro(CreateLegacyAlias _cmake_target _old_path _new_name)
	add_custom_target("${_cmake_target}" ALL
		COMMAND "${CMAKE_COMMAND}" -E create_symlink "${_new_name}" "${_old_path}"
		WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
	add_dependencies("${_cmake_target}" "${_new_name}")
	install(PROGRAMS "${_old_path}" DESTINATION ${BINDIR})
endmacro()

install(TARGETS ${PROJECT_NAME}server DESTINATION ${BINDIR})

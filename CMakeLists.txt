cmake_minimum_required(VERSION 3.12)

# This can be read from ${PROJECT_NAME} after project() is called
project(luanti)
set(PROJECT_NAME_CAPITALIZED "Luanti")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)
set(GCC_MINIMUM_VERSION "7.5")
set(CLANG_MINIMUM_VERSION "7.0.1")

# You should not need to edit these manually, use util/bump_version.sh
set(VERSION_MAJOR 5)
set(VERSION_MINOR 11)
set(VERSION_PATCH 0)
set(VERSION_EXTRA "" CACHE STRING "Stuff to append to version string")

# Change to false for releases
set(DEVELOPMENT_BUILD TRUE)

set(VERSION_STRING "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")
if(VERSION_EXTRA)
	set(VERSION_STRING "${VERSION_STRING}-${VERSION_EXTRA}")
elseif(DEVELOPMENT_BUILD)
	set(VERSION_STRING "${VERSION_STRING}-dev")
endif()

if (CMAKE_BUILD_TYPE STREQUAL Debug)
	# Append "-debug" to version string
	set(VERSION_STRING "${VERSION_STRING}-debug")
endif()

# Configuration options
set(BUILD_DOCUMENTATION TRUE CACHE BOOL "Build documentation")

set(DEFAULT_ENABLE_LTO TRUE)
# by default don't enable on Debug builds to get faster builds
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(DEFAULT_ENABLE_LTO FALSE)
endif()
set(ENABLE_LTO ${DEFAULT_ENABLE_LTO} CACHE BOOL "Use Link Time Optimization")

set(BUILD_WITH_TRACY FALSE CACHE BOOL
	"Fetch and build with the Tracy profiler client")
set(FETCH_TRACY_GIT_TAG "master" CACHE STRING
	"Git tag for fetching Tracy client. Match with your server (gui) version")

message(STATUS "*** Will build version ${VERSION_STRING} ***")
message(STATUS "BUILD_DOCUMENTATION: " ${BUILD_DOCUMENTATION})

set(WARN_ALL TRUE CACHE BOOL "Enable -Wall for Release build")

if(NOT CMAKE_BUILD_TYPE)
	# Default to release
	set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type: Debug or Release" FORCE)
endif()

set(ENABLE_UPDATE_CHECKER (NOT ${DEVELOPMENT_BUILD}) CACHE BOOL
	"Whether to enable update checks by default")

# Included stuff
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")

if(TRUE)
	message(STATUS "Using imported IrrlichtMt at subdirectory 'irr'")
	add_library(IrrlichtMt::IrrlichtMt INTERFACE IMPORTED)
	set_target_properties(IrrlichtMt::IrrlichtMt PROPERTIES
		INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_SOURCE_DIR}/irr/include")
endif()

if (ENABLE_LTO OR CMAKE_INTERPROCEDURAL_OPTIMIZATION)
	include(CheckIPOSupported)
	check_ipo_supported(RESULT lto_supported OUTPUT lto_output)
	if(lto_supported)
		set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
		message(STATUS "LTO/IPO is enabled")
	else()
		set(CMAKE_INTERPROCEDURAL_OPTIMIZATION FALSE)
		message(STATUS "LTO/IPO was requested but is not supported by the compiler: ${lto_output}")
	endif()
else()
	message(STATUS "LTO/IPO is not enabled")
endif()

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
	if(CMAKE_CXX_COMPILER_VERSION VERSION_LESS "${GCC_MINIMUM_VERSION}")
		message(FATAL_ERROR "Insufficient gcc version, found ${CMAKE_CXX_COMPILER_VERSION}. "
			"Version ${GCC_MINIMUM_VERSION} or higher is required.")
	endif()
endif()

# Installation

set(SHAREDIR ".")
set(BINDIR "bin")
set(DOCDIR "doc")
set(EXAMPLE_CONF_DIR ".")
set(MANDIR "unix/man")
set(XDG_APPS_DIR "unix/applications")
set(METAINFODIR "unix/metainfo")
set(ICONDIR "unix/icons")
set(LOCALEDIR "locale")

set(CUSTOM_SHAREDIR "" CACHE STRING "Directory to install data files into")
if(NOT CUSTOM_SHAREDIR STREQUAL "")
	set(SHAREDIR "${CUSTOM_SHAREDIR}")
	message(STATUS "Using SHAREDIR=${SHAREDIR}")
endif()

set(CUSTOM_BINDIR "" CACHE STRING "Directory to install binaries into")
if(NOT CUSTOM_BINDIR STREQUAL "")
	set(BINDIR "${CUSTOM_BINDIR}")
	message(STATUS "Using BINDIR=${BINDIR}")
endif()

set(CUSTOM_DOCDIR "" CACHE STRING "Directory to install documentation into")
if(NOT CUSTOM_DOCDIR STREQUAL "")
	set(DOCDIR "${CUSTOM_DOCDIR}")
	message(STATUS "Using DOCDIR=${DOCDIR}")
endif()

set(CUSTOM_MANDIR "" CACHE STRING "Directory to install manpages into")
if(NOT CUSTOM_MANDIR STREQUAL "")
	set(MANDIR "${CUSTOM_MANDIR}")
	message(STATUS "Using MANDIR=${MANDIR}")
endif()

set(CUSTOM_EXAMPLE_CONF_DIR "" CACHE STRING "Directory to install example config file into")
if(NOT CUSTOM_EXAMPLE_CONF_DIR STREQUAL "")
	set(EXAMPLE_CONF_DIR "${CUSTOM_EXAMPLE_CONF_DIR}")
	message(STATUS "Using EXAMPLE_CONF_DIR=${EXAMPLE_CONF_DIR}")
endif()

set(CUSTOM_XDG_APPS_DIR "" CACHE STRING "Directory to install .desktop files into")
if(NOT CUSTOM_XDG_APPS_DIR STREQUAL "")
	set(XDG_APPS_DIR "${CUSTOM_XDG_APPS_DIR}")
	message(STATUS "Using XDG_APPS_DIR=${XDG_APPS_DIR}")
endif()

set(CUSTOM_ICONDIR "" CACHE STRING "Directory to install icons into")
if(NOT CUSTOM_ICONDIR STREQUAL "")
	set(ICONDIR "${CUSTOM_ICONDIR}")
	message(STATUS "Using ICONDIR=${ICONDIR}")
endif()

set(CUSTOM_LOCALEDIR "" CACHE STRING "Directory to install l10n files into")
if(NOT CUSTOM_LOCALEDIR STREQUAL "")
	set(LOCALEDIR "${CUSTOM_LOCALEDIR}")
	message(STATUS "Using LOCALEDIR=${LOCALEDIR}")
endif()


install(DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/builtin" DESTINATION "${SHAREDIR}")
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/mods/mods_here.txt" DESTINATION "${SHAREDIR}/mods")
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/textures/texture_packs_here.txt" DESTINATION "${SHAREDIR}/textures")

set(INSTALL_DEVTEST FALSE CACHE BOOL "Install Development Test")

if(INSTALL_DEVTEST)
	install(DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/games/devtest" DESTINATION "${SHAREDIR}/games/"
		PATTERN ".git*" EXCLUDE )
endif()

install(FILES "README.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "doc/lua_api.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "doc/client_lua_api.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "doc/menu_lua_api.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "doc/texture_packs.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "doc/world_format.md" DESTINATION "${DOCDIR}" COMPONENT "Docs")
install(FILES "minetest.conf.example" DESTINATION "${EXAMPLE_CONF_DIR}")

install(FILES "doc/luanti.6" "doc/luantiserver.6" DESTINATION "${MANDIR}/man6")
install(FILES "misc/net.minetest.minetest.desktop" DESTINATION "${XDG_APPS_DIR}")
install(FILES "misc/net.minetest.minetest.metainfo.xml" DESTINATION "${METAINFODIR}")
install(FILES "misc/luanti.svg" DESTINATION "${ICONDIR}/hicolor/scalable/apps")
install(FILES "misc/luanti-xorg-icon-128.png"
	DESTINATION "${ICONDIR}/hicolor/128x128/apps"
	RENAME "luanti.png")

# Library pack
find_package(GMP REQUIRED)
find_package(Json 1.0.0 REQUIRED)
find_package(Lua REQUIRED)
if(NOT USE_LUAJIT)
	add_subdirectory(lib/bitop)
endif()
add_subdirectory(lib/sha256)

add_subdirectory(lib/tiniergltf)

# Subdirectories
# Be sure to add all relevant definitions above this
add_subdirectory(src)

# CPack

set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "A free open-source voxel game engine with easy modding and game creation.")
set(CPACK_PACKAGE_VERSION_MAJOR ${VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${VERSION_PATCH})
set(CPACK_PACKAGE_VENDOR "celeron55")
set(CPACK_PACKAGE_CONTACT "Perttu Ahola <celeron55@gmail.com>")

include(CPackComponent)

cpack_add_component(Docs
	DISPLAY_NAME "Documentation"
	DESCRIPTION "Documentation about ${PROJECT_NAME_CAPITALIZED} and ${PROJECT_NAME_CAPITALIZED} modding"
)

set(CPACK_PACKAGE_FILE_NAME "${PROJECT_NAME}-${VERSION_STRING}-linux")
set(CPACK_GENERATOR TGZ)
set(CPACK_SOURCE_GENERATOR TGZ)

include(CPack)

# Add a target to generate API documentation with Doxygen
if(BUILD_DOCUMENTATION)
	find_package(Doxygen)
	if(DOXYGEN_FOUND)
		configure_file(${CMAKE_CURRENT_SOURCE_DIR}/doc/Doxyfile.in
				${CMAKE_CURRENT_BINARY_DIR}/doc/Doxyfile @ONLY)
		add_custom_target(doc
			${DOXYGEN_EXECUTABLE} ${CMAKE_CURRENT_BINARY_DIR}/doc/Doxyfile
			WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/doc
			COMMENT "Generating API documentation with Doxygen" VERBATIM
		)
	endif()
endif()

# Fetch Tracy
if(BUILD_WITH_TRACY)
	include(FetchContent)

	message(STATUS "Fetching Tracy (${FETCH_TRACY_GIT_TAG})...")
	FetchContent_Declare(
		tracy
		GIT_REPOSITORY https://github.com/wolfpld/tracy.git
		GIT_TAG ${FETCH_TRACY_GIT_TAG}
		GIT_SHALLOW TRUE
		GIT_PROGRESS TRUE
	)
	FetchContent_MakeAvailable(tracy)
	message(STATUS "Fetching Tracy - done")
endif()

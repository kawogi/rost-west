if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

# If the user is cross compiling to a different target, update the location of the target directory
# *Note*: Make sure to install the target with rustup, as well as setting the proper environment variables, such as CC, HOST_CC, CARGO_TARGET_<triple>_LINKER, ...
# See https://doc.rust-lang.org/cargo/reference/environment-variables.html#configuration-environment-variables
if (DEFINED ENV{CARGO_BUILD_TARGET})
    set(TARGET_DIR "$ENV{CARGO_BUILD_TARGET}/${TARGET_DIR}")
endif()

if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-17" "-Clink-arg=-fuse-ld=lld-17")
endif()

set(ROST_WEST_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librost_west.a")

set(ROST_WEST_CXX "${CMAKE_CURRENT_BINARY_DIR}/rost_west.cpp")
add_library(rost_west STATIC ${ROST_WEST_CXX})
add_custom_command(
    OUTPUT ${ROST_WEST_CXX}
    DEPENDS
        ${CMAKE_CURRENT_BINARY_DIR}/src/*
        # ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rost_west/src/lib.rs.cc
        # ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rost_west/src/lib.rs.h
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rost_west/src/lib.rs.cc ${ROST_WEST_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rost_west/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/rost_west.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(rost_west pthread dl ${ROST_WEST_LIB})

add_test(NAME rost_west_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

#!/bin/bash -e

cmake -B build \
	-DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE:-Debug} \
	-DENABLE_LTO=FALSE \
	${CMAKE_FLAGS}

cmake --build build --parallel $(($(nproc) + 1))

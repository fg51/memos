# cmake version
cmake_minimum_required(VERSION 3.21)

# project info
project(
  example
  VERSION 0.0.1
  DESCRIPTION "description"
  LANGUAGES C)

# in-source build-guard
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(
    FATAL_ERROR
      "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from ther. You may need to remove CMakeCache.txt. "
  )
endif()

# read CMakeLists.txt in sub-directory.
#
# add_subdirectory(src/lib)
add_subdirectory(src/bin EXCLUDE_FROM_ALL)

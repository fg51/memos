# gen compile_commands.json

```CMakeLists.txt
# cmake version
cmake_minimum_required(VERSION 3.21)

# project info
project(name
  VERSION 0.0.5
  DESCRIPTION "description"
  LANGUAGES C
)

# in-source build-guard
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from ther. You may need to remove CMakeCache.txt. ")
endif()

# read CMakeLists.txt in sub-directory.
add_subdirectory(src/lib)
add_subdirectory(src/bin EXCLUDE_FROM_ALL)
```


## subdirectory
```CMakeLists.txt
# build option
# useage: $ cmake .. -DMYLIB_BUILD_SHARED_LIBS=ON
# option(MYLIB_BUILD_SHARED_LIBS "build mylib as a shared library" OFF)
#
# if (MYLIB_BUILD_SHARED_LIBS)
#    add_library(mylib1 SHARED)
#  else()
add_library(mylib1 STATIC)
# endif()

# required source
target_sources(mylib
  PRIVATE
  myliba.c
  mylibb.c
)

target_include_directories(mylib1
  PUBLIC
  ${PROJECT_SOURCE_DIR}/include
)
target_include_directories(mylib1
  PRIVATE
  ${PROJECT_SOURCE_DIR}/mock
)

target_compile_options(mylib1
  PUBLIC -O2 -Wall -Wextra
)
```

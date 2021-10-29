CMAKE
====

[C/C++プロジェクトをCMakeでビルドする - Qiita](https://qiita.com/Hiroya_W/items/049bfb4c6ad3dfe6ff0c)


```sh
$ mkdir build & cd build
$ cmake -GNinja ..
$ ninja build
```


# trace 
```sh
$ cmake .. --trace
```

# verbose
```sh
$ make VERBOSE=1
```

# ROOT
```cmake@CMakeLists.txt
# cmake version
cmake_minimum_required(VERSION 3.21)

# project info
project(mylib
  VERSION 1.0
  DESCRIPTION "mylib project"
  LANGUAGES C
  # set(CMAKE_C_COMPILER clang)
)

# in-source build-guard
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from ther. You may need to remove CMakeCache.txt. ")
endif()

# read CMakeLists.txt in sub-directory.
add_subdirectory(lib/mylib)
add_subdirectory(examples EXCLUDE_FROM_ALL)
```

# LIB

```cmake
# build option
# useage: $ cmake .. -DMYLIB_BUILD_SHARED_LIBS=ON
option(MYLIB_BUILD_SHARED_LIBS "build mylib as a shared library" OFF)

if (MYLIB_BUILD_SHARED_LIBS)
  add_library(mylib SHARED)
else()
  add_library(mylib STATIC)
endif()

# required source
target_sources(mylib
  PRIVATE
  mylib.c
)

target_include_directories(mylib
  PUBLIC
  ${PROJECT_SOURCE_DIR}/include
)

# read the variable: CMAKE_INSTALL_<dir> defined GNU-directory
include(GNUInstallDirs)

# install mylib library
install(TARGETS mylib
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

# install headers
install(DIRECTORY ${CMAKE_SOURCE_DIR}/include/
  DESTINATION ${CMAKE_INSTALL__INCLUDEDIR}
  FILES_MATCHING PATTERN "*.h"
)
```

# examples/CMakeLists.txt
```cmake
add_custom_target(examples)
add_dependencies(examples
  main
)

add_executable(main main.c)
target_link_libraries(main
  mylib
)
```

# set compiler
```sh
$ cmake .. -DCMAKE_C_COMPILER=/usr/bin/clang -DCMAKE_CXX_COMPILER=/usr/bin/clang++
$ cmake .. -DCMAKE_BUILD_TYPE=Debug
$ cmake .. -DCMAKE_BUILD_TYPE=Release
```


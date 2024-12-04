# cmake presets

touch CMakePresets.json

# usage

```sh
$ cmake --preset ${preset_name} --fresh
```

```json
{
  "version": 3,
  "configurePresets": [
    {
      "name": "default",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Release",
        "CMAKE_C_STANDARD": "11",
        "CMAKE_C_FLAGS": "-Wall -Wextra"
      }
    },
    {
      "name": "debug",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build-debug",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Debug",
        "MY_CUSTOM_OPTION": "ON"
      }
    }
  ]
}
```

```json
{
  "version": 3,
  "configurePresets": [
    {
      "name": "default",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Release",
        "CMAKE_CXX_STANDARD": "17",
        "CMAKE_CXX_FLAGS": "-Wall -Wextra"
      }
    },
    {
      "name": "debug",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build-debug",
      "cacheVariables": {
        "CMAKE_BUILD_TYPE": "Debug",
        "MY_CUSTOM_OPTION": "ON"
      }
    }
  ]
}
```

# usage
```sh
$ gn gen $DIR_NAME
$ ninja -C $DIR_NAME $TARGET
$ ./$DIR_NAME/$TARGET
```

## example
```sh
$ ninja -C out some/path/to/target:my_target
```

# directory
.
├── build
│   ├── BUILDCONFIG.gn
│   ├── BUILD.gn
│   └── toolchain
│       └── BUILD.gn
│
├── .gn
├── BUILD.gn
│
├── hello.cc
├── hello_shared.cc
├── hello_shared.h
├── hello_static.cc
└── hello_static.h

# flow
1. "gn" read the path: "buildconfig.gn" from ".gn".
2. "buildconfig.gn" read the "BUILD.gn" files.

"${//path/to/dir}":"command"



# 1. create project
```sh
$ mkdir proj && cd proj
$ zig init-exe
```

## 1.1 build.zig

### with c-src

```zig
const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    //const lib = b.addStaticLibrary("ex4-cimport", "src/main.zig");
    const exe = b.addStaticLibrary("a.out", "src/main.zig");
    exe.addCSourceFile("c-src/sub.c", &[_][]const u8{"-std=c99"});
    exe.setBuildMode(mode);

    exe.addIncludeDir("c-src");
    exe.linkSystemLibrary("c");
    exe.install();

    const main_tests = b.addTest("src/main.zig");
    main_tests.addCSourceFile("c-src/sub.c", &[_][]const u8{"-std=c99"});
    main_tests.addIncludeDir("c-src");
    main_tests.linkSystemLibrary("c");
    main_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
```

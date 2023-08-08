const std = @import("std");
const Builder = std.build.Builder;

pub fn build(b: *Builder) void {
  const mode = b.standardReleaseOptions();
  const windows = b.option(bool, "windows", "create windows build") orelse false;
  const vcpkg = b.option(bool, "vcpkg", "Add vcpkg paths to the build") orelse false;

  var exe = b.addExecutable("a.out", "src/main.zig");
  exe.addCSourceFile("stb_image/stb_image_impl.c", &[_][]const u8{"std=c99"});
  exe.setBuildMode(mode);

  if (windows) {
    exe.setTarget(.{
      .cpu_arch = .x86_64,
      .os_tag = .windows,
      .abi = .gnu,
    });
  }

  if (vcpkg) {
    exe.addVcpkgPaths(.static) catch @panic("Cannot add vcpkg paths.");
  }

  exe.addIncludeDir("std_image-2.22");

  exe.linkSystemLibrary("c");
  exe.install();
}

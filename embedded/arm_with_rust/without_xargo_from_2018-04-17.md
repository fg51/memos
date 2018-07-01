
```sh
$ rustup show
$ rustup target list | ag thumbv
```


```sh
$ rustup target add thumbv7em-none-eabihf
```

* thumbv6m-none-eabi	Targets the Cortex-M0, Cortex-M0+ and Cortex-M1 processors (ARMv6-M architecture)
* thumbv7em-none-eabi	Targets the Cortex-M4 and Cortex-M7 processors (ARMv7E-M)
* thumbv7em-none-eabihf	Targets the Cortex-M4F and Cortex-M7F processors (ARMv7E-M)
* thumbv7m-none-eabi	Targets the Cortex-M3 processor (ARMv7-M)

```sh
$ cargo new sample-prj
```

## setup linker

use "lld"

```sh
$ cat .cargo/config
[target.thumbv7em-none-eabihf]
runner = 'arm-none-eabi-gdb'
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "linker=lld",
  "-Z", "linker-flavor=ld.lld",
  "-Z", "thinlto=no",
]
```

## setup crate

```sh
$ cat Cargo.toml
[package]
name = "sample-prj"
authors = ["your name"]
version = "0.0.1"

[dependencies]
cortex-m = "0.4.0"
cortex-m-rt = "0.4.0"
panic-abort = "0.1.1"
volatile-register = "0.2.0"
```

## build

```sh
$ cargo build --target thumbv7em-none-eabihf
$ file sample-prj
... : ELF 32-bit LSB executable, ARM, EABI ...
```

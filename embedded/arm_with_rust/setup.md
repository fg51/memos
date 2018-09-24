Arm with Rust
====

[Rust your ARM microcontroller! | Embedded in Rust](http://blog.japaric.io/quickstart/)


```sh
$ # Switch to the nightly channel
$ rustup default nightly-2017-04-24

$ rustc -V
rustc 1.18.0-nightly (2bd4b5c6d 2017-04-23)

$ sudo pacman -S arm-none-eabi-binutils arm-none-eabi-gdb openocd

$ arm-none-eabi-ld -V | head -n1
GNU ld (GNU Binutils) 2.28

$ arm-none-eabi-gdb -v | head -n1
GNU gdb (GDB) 7.12.1

$ openocd -v 2>&1 | head -n1
Open On-Chip Debugger 0.10.0

$ cargo install xargo

$ xargo -V
xargo 0.3.6
cargo 0.19.0-nightly (8326a3683 2017-04-19)

$ # for Xargo
$ rustup component add rust-src
$ rustup target list
$ rustc --print target-list
...
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
...
```


## xargo

at cross-compile, use xargo instead of cargo.

```sh
$ xargo new prject_name --bin
```




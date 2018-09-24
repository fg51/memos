
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
$ file ./target/thumbv7em-none-eabihf/debug/sample-prj
... : ELF 32-bit LSB executable, ARM, EABI ...
```

## strip

```sh
$ file ./a.out
$ arm-linux-gnueabihf-strip ./a.out
$ ls -lh *.out
```

## flush

### connect

```sh
$ openocd -f board/st_nucleo_f4.cfg
$ openocd -f interface/stling-v2-1.cfg -f target/stm32f4x.cfg
```

### flush

```sh
$ telnet localhost 4444
> reset halt
> flash write_image erase ${elf_file_absolute_path}
```

### with config

```sh
$ cat openocd.cfg
telnet_port 4444
gdb_port 3333

source [find board/st_nucleo_f4.cfg]

init

proc flash_elf {elf_file} {
  reset
  halt
  flash write_image erase $elf_file
  verify_image $elf_file
  echo "flash write_image ($elf_file) complete"
  reset
  exit
}

$ openocd -f openocd.cfg -c "flash_elf ${elf_file_relative_path}
```


## example

```rust
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]


#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use rt::ExceptionFrame;


entry!(main);

fn main() -> ! {
    loop {}
}


exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}


exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}
```

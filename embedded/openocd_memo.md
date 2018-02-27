OPENOCD
====

## install

### depends

```sh
$ sudo pacman -S libftdi libftdi-compat libusb libusb-compat hidapi
```

### download

$ wget https://downloads.sourceforge.net/sourceforge/openocd/openocd-0.10.0.tar.bz2
$ tar xvf openocd-0.10.0.tar.bz2 && cd openocd-0.10.0
$ sed -i 's|ftdi_new();|(void*)12345;|g' configure{,.ac}

### build

$ libtoolize
$ autoreconf
$ ./configure --prefix=/usr ${_features[@]/#/--enable-} --disable-werror
$ make




## create GDB server

```sh
$ cd /path/to/openocd/tcl
$ ../src/openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
```

### connect test with telnet

```sh
$ telnet localhost 4444
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Open On-Chip Debugger

> reset halt
Unable to match requested speed 2000 kHz, using 1800 kHz
Unable to match requested speed 2000 kHz, using 1800 kHz
adapter speed: 1800 kHz

target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x0800053f0 msp: 0x20000c78

> reset
>
```

### telnet commands

- halt : stop mpu
- reset : restart mpu
- ctrl + ] : exit telnet
- ctrl + d : exit telnet


###

```sh
$ cd /path/to/project_with_rust
$ arm-none-eabi-gdb target/thumv7em-none-eabihf/debug/examples/hello
> tbreak hello::main()
> continue
```


- continue: continue or c

STM 32 discovery
========================================

[Ubuntu(Linux)環境のSTM32(ARM)開発環境についてのメモ その1](http://memo.tank.jp/archives/9977)
[Ubuntu(Linux)環境のSTM32(ARM)開発環境についてのメモ その2](http://memo.tank.jp/archives/10052)

1. install arm toolchain
2. install stlink
3. install HID for CMSIS-DAP
4. install OpenOCD
5. install eclipse (for template)


## 1. LaunchPad GNU ARM Emdedded Toolchain

2015 ubuntu 14.04LTS (trusty)

[terry guo launchpad](https://launchpad.net/~terry.guo/+archive/ubuntu/gcc-arm-embedded)

```sh
$ sudo apt-get remove binutils-arm-none-eabi gcc-arm-none-eabi
$ sudo add-apt-repository ppa:terry.guo/gcc-arm-embedded
$ sudo apt-get update
$ sudo apt-get install -y gcc-arm-none-eabi=4.9.3.2015q3-1trusty1
```

old: $ sudo apt-get install gcc-arm-none-eabi=4.9.3.2015q1-0trusty13


$ arm-none-eabi-gcc -v
$ arm-none-eabi-gcc -print-multi-lib


### CAUTION

"gcc-arm-none-eabi" of "LaunchPad" is conflicted with one of the system.

`dist-upgrade ` とすると、
システムが用意した gcc-arm-none-eabi が復活するので注意


### DO NOT USE

```sh
$ sudo apt-get install binutils-arm-none-eabi gcc-arm-none-eabi
```

”/usr/arm-none-eabi/include/c++”のヘッダファイルがなかったりライブラリィが不足してたりと中途半端で、mbedのソースがコンパイルできないのである。


### uninstall original binutils

```sh
$ sudo apt-get remove binutils-arm-none-eabi gcc-arm-none-eabi
```


## 2. debug tool

### ST-link
[use st-link](http://wiki.tokor.org/index.php?Linux上でSTM32のプログラミング・デバッグ環境を整える%u3000-%u3000STM32)

```sh
$ git clone https://github.com/texane/stlink
$ cd stlink
$ ./autogen.sh
$ ./configure
$ make
$ sudo make install     # enable use without sudo
$ sudo cp 49-stlinkv*.rules /etc/udev/rules.d
$ sudo udevadm control --reload-rules
```

```
at libusb missing,
$ sudo apt-get install libusb-1.0-0-dev
```

installed command: st-flash, st-info, st-term, st-util


#### start the GDB server at port:4242

```sh
$ st-util
2015-05-21T21:14:05 INFO src/stlink-usb.c: -- exit_dfu_mode
2015-05-21T21:14:05 INFO src/stlink-common.c: Loading device parameters....
2015-05-21T21:14:05 INFO src/stlink-common.c: Device connected is: F4 device, id 0x10016413
2015-05-21T21:14:05 INFO src/stlink-common.c: SRAM size: 0x30000 bytes (192 KiB), Flash: 0x100000 bytes (1024 KiB) in pages of 16384 bytes
2015-05-21T21:14:05 INFO gdbserver/gdb-server.c: Chip ID is 00000413, Core ID is  2ba01477.
2015-05-21T21:14:05 INFO gdbserver/gdb-server.c: Target voltage is 2886 mV.
2015-05-21T21:14:05 INFO gdbserver/gdb-server.c: Listening at *:4242...
```


#### start the GDB with binary

build, connect, load, and execute:

```sh
$ arm-none-eabi-gdb <file name.elf>
> target extended-remote localhost:4242
> load
> continue
```

### OpenOCD
to see the OpenOCD.


#### OpenOCD

[CMSIS-DAP from OpenOCD](http://nemuisan.blog.bai.ne.jp/?eid=207857)
[wiki](http://www.kimura-lab.net/wikimura/index.php/OpenOCDが動くまで)


1. HID device libs for CMSIS-DAP

[HID API for inux](http://www.signal11.us/oss/hidapi/)

```sh
$ sudo apt-get install libtool libudev-dev autoconf libusb-dev libusb-1.0-0-dev
$ git clone http://github.com/signal11/hidapi.git
$ cd hidapi/
$ ./bootstrap
$ ./configure
$ make
$ sudo make install
$ sudo ln -s /usr/local/lib/libhidapi-hidraw.so.0 /usr/lib/libhidapi-hidraw.so.0
$ sudo cp udev/99-hid.rules /etc/udev/rules.d/

#$ sudo vim /etc/udev/rules.d/99-hidraw-permissions.rules
#+ KERNEL==”hidraw*”, SUBSYSTEM==”hidraw”, MODE=”0664″, GROUP=”plugdev”
```


2. OpenOCD
[stm32f3 discovery dev in ubuntu](http://mohammedari.blogspot.jp/2013/11/ubuntustm32f3-discovery.html)

```sh
$ git clone git://openocd.git.sourceforge.net/gitroot/openocd/openocd
$ cd openocd/
$ ./bootstrap
$ ./configure --enable-maintainer-mode --enable-cmsis-dap --enable-hidapi-libusb --enable-stlink
$ make
$ sudo make install
$ sudo cp /usr/local/share/openocd/contrib/99-openocd.rules /etc/udev/rules.d/
$ sudo udevadm control --reload-rules
$ sudo udevadm trigger
$ openocd --version
```

```sh: openocd connect stm32 and start server
$ openocd -f /usr/local/share/openocd/scripts/board/stm32fdiscovery.cfg
```

```sh: telnet openocd server
$ telnet localhost:4444
$ help
```


#### commands:
* $ halt
* $ poll
* $ flash probe 0
* $ flash write\_image erase main.out 0x0800000000 elf


#### session end

$ ctrl + ]


#### telnet quit

$ c (ctrl + Enter)
$ q (ctrl + Enter)


#### caution

OpenOCD経由でターゲットボードにイメージを書きこむ際、

```sh
$ flash write_image erase path/to/image.elf
```

この時のpathは, OpenOCD起動時の.cfgファイルからの相対パスとなります。

また、/home/以下などにイメージファイルがある場合、パーミッションがなくてOpenOCDがイメージファイルを開くことができません。
そのため、イメージファイルをどこか適当な場所に配置するか、OpenOCDをroot権限で起動する必要があります。



### Eclipse

1. download Eclipse IDE for Java Developers

[eclipse downloads](https://eclipse.org/downloads/)


2. sudo mkdir /opt/eclipse && cd /opt/eclipse
3. tar xavf ~/Downloads/eclipse-\*
4. sudo ln -s /opt/eclipse/eclipse /usr/bin



#### uninstall

```sh
$ eclipse -clean
$ sudo rm -rf /opt/eclipse
```


#### install plugin

help(H) > install new software > add(A) 

http://gnuarmeclipse.sourceforge.net/updates

check all


#### create

file(F) > new(n) > project > C/C++ > C Project 
 > STM32F4xx C/C++ Project

STM32F407xx
flash size: 1024
clock: 8000000
use system calls ?: freestanding(no POSIX system calls)



#### build option

```sh
arm-none-eabi-g++ \
    -mcpu=cortex-m4 -mthumb -mfloat-abi=soft \
    -Og -fmessage-length=0 -fsigned-char -ffunction-sections -fdata-sections \
    -ffreestanding -fno-move-loop-invariants \
    -Wunused -Wuninitialized -Wall -Wextra -Wmissing-declarations \
    -Wconversion -Wpointer-arith -Wpadded -Wshadow -Wlogical-op \
    -Waggregate-return -Wfloat-equal  \
    -g3 \
    -T mem.ld -T libs.ld -T sections.ld \
    -nostartfiles -Xlinker --gc-sections \
    -L"../ldscripts" -Wl,-Map,"ref_stm32_01.map" \
    --specs=nano.specs \
    -o "ref_stm32_01.elf" $(OBJS) $(USER_OBJS) $(LIBS)
```


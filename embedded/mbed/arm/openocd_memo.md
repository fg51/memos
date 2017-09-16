OpenOCD
========================================

[CMSIS-DAP from OpenOCD](http://nemuisan.blog.bai.ne.jp/?eid=207857)
[wiki](http://www.kimura-lab.net/wikimura/index.php/OpenOCDが動くまで)


## 1. require-install HID device libs for CMSIS-DAP

[HID API for inux](http://www.signal11.us/oss/hidapi/)

CMSIS-DAP need the HID device libs: hidapi.

```bash
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


## 2. install OpenOCD

[stm32f3 discovery dev in ubuntu](http://mohammedari.blogspot.jp/2013/11/ubuntustm32f3-discovery.html)

```bash
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

### set stm32

```bash: openocd connect stm32 and start server
$ openocd -f /usr/local/share/openocd/scripts/board/stm32fdiscovery.cfg
```

```bash: telnet openocd server
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

```bash
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

```bash
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

arm-none-eabi-g++ 
    -mcpu=cortex-m4 -mthumb -mfloat-abi=soft 
    -Og -fmessage-length=0 -fsigned-char -ffunction-sections -fdata-sections 
    -ffreestanding -fno-move-loop-invariants 
    -Wunused -Wuninitialized -Wall -Wextra -Wmissing-declarations 
    -Wconversion -Wpointer-arith -Wpadded -Wshadow -Wlogical-op 
    -Waggregate-return -Wfloat-equal  
    -g3 
    -T mem.ld -T libs.ld -T sections.ld 
    -nostartfiles -Xlinker --gc-sections 
    -L"../ldscripts" -Wl,-Map,"ref_stm32_01.map" 
    --specs=nano.specs 
    -o "ref_stm32_01.elf" $(OBJS) $(USER_OBJS) $(LIBS)



STM32
====

[Linux上でSTM32のプログラミング・デバッグ環境を整える　-　STM32](http://wiki.tokor.org/index.php?Linux上でSTM32のプログラミング・デバッグ環境を整える　-　STM32)


## install

get arm-none-eabi-gcc arm-none-eabi-gdb
[launchpad](https://launchpad.net/gcc-arm-embedded)


in arch linux
```sh
$ sudo pacman -Syu openocd stlink
$ packer -Sy  stm32flash-git
```

donot use:
$ sudo pacman -Syu arm-none-eabi-gcc arm-none-eabi-gdb


## ST-Link

texane/st-link

```sh
$ sudo apt-get install libusb-1.0-0-dev
$ git clone https://github.com/texane/stlink stlink.git
$ cd stlink-master
$ ./autogen.sh
$ ./configure
$ make
$ sudo paco -D make install 
$ sudo cp *.rules /etc/udev/
$ sudo restart udev
```


### usgae command

```sh
$ st-flash
$ st-info
$ st-term
$ st-util
```


## flash
```sh
$ sudo ./flash/st-flash write <src>.bin 0x8000000

$ cp flash/st-flash /usr/bin

$ sudo st-flash write <dest> <src>.bin <address> ??

#for stlink v1
$ ./st-flash write v1 <org>.bin 0x8000000

#for stlink v2
$ ./st-flash write <org>.bin 0x8000000
```



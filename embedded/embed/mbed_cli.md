mbed-cli
====

[mbed OS 5 の CLI 開発環境を Linux 上に構築する - Qiita](http://qiita.com/hotchpotch/items/4d87c13b97d236db5b4f)

## install

```sh
$ pip2 install mbed-cli
```

## sample import

```sh
$ mbed import https://developer.mbed.org/teams/mbed-os-examples/code/mbed-os-example-blinky
$ cd mbed-os-example-blinky
$ mbed detect

$ ls .
.gitignore
.hg/
.mbed
img/
main.cpp
mbed-os/
mbed-os.lib
mbed_settings.py
README.md
```

* mbed_settings.py: config file

GCC

```sh
$ mbed target <target cpu>
$ mbed toolchain GCC_ARM
$ mbed compile
```



mbed import http

```sh
$ mbed import https://developer.mbed.org/teams/mbed/code/mbed_blinky/
$ mbed deploy   # for library
```

$ mbed import https://developer.mbed.org/teams/mbed/code/mbed_blinky/

mbed OS: https://github.com/ARMmbed/mbed-os-example-blinky


## create project

```sh
$ mbed new <proj>
```

## mount mbed

```sh
$ sudo mount -t vfat /dev/sdb /mnt/usb
$ mbed detect
```


## build

```sh
$ mbed_settings.py
  # GCC ARM
- #GCC_ARM_PATH = ""
+ GCC_ARM_PATH = "/opt/gcc-arm-none-eabi-5_3_2016q1/bin"
```

```sh
$ mbed compile -m NUCLEO_F446RE
```


## serial communication

```sh
$ minicom -s
```


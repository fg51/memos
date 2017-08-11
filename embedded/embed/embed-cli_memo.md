mbed-cli
====

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

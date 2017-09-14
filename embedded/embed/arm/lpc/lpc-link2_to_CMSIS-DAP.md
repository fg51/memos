lpc-link2
====


## what is CMSIS-DAP

[CMSIS-DAPって何？](http://rx.tokudenkairo.co.jp/cmsisdap/cmsisdap.html)


## LPCScrypt

[LPCScrypt | www.LPCware.com](https://www.lpcware.com/lpcscrypt)

```sh@ubuntu
$ sudo dpkg --add-architecture i386
$ sudo apt-get update

$ sudo apt-get install -y libc6:i386 libusb-dev:i386 uuid-dev:i386
$ apt-get install usbutils  #NOTE: lsusb
```


## to CMSIS-DAP

write the firm: CMSIS-DAP in the board: lpc-link2 via LPCScrypt

1. boot from USB0 (DFU mode): JP1 not fitted

2. write the firm

```sh
$ /path/to/LPCScrypt/scripts/program_CMSIS.cmd


```

3. reboot from SPIFI (SPI-Flash) : JP1 fitted

NOTE: MCU on the LPC-Link2 has no internal flash. (but has SPI-Flash)


## lpcxpresso

## require

```sh@ubuntu
$ sudo dpkg --add-architecture i386
$ sudo apt-get update

$ sudo apt-get install libc6:i386 libusb-dev:i386 uuid-dev:i386

# for gui
$ sudo apt-get install libgtk2.0-0:i386 libxtst6:i386 libpangox-1.0-0:i386 \
libpangoxft-1.0-0:i386 libidn11:i386 libglu1-mesa:i386 \
libncurses5:i386 libudev1:i386 libusb-1.0:i386 libusb-0.1:i386 \
gtk2-engines-murrine:i386 libnss3-1d:i386 libwebkitgtk-1.0-0
```

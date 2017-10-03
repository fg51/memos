dspic
========================================

[ref](http://matoken.org/blog/blog/2012/08/31/ubuntu-12-04-mplabx)

[compire](https://microchip.wikidot.com/xc16:installation)

[microchip.wikidot mplabx](http://microchip.wikidot.com/install:mplabx-lin64)

[microchip.wikidot xc16](https://microchip.wikidot.com/xc16:installation)

[MPLAB® XC Compilers | Microchip Technology Inc.](http://www.microchip.com/pagehandler/en_us/devtools/mplabxc/)


## XC16 (compire)

```sh
$ chmod u+x xc16-vX.XX-linux-installer.run
$ sudo ./xc16-vX.XX-linux-installer.run --help
$ sudo ./xc16-vX.XX-linux-installer.run --mode text
```

gcc-libslib32-gcc-libs
sdx
lib32-tclkit
tclkit


## peripheral

download: peripheral-libraries-for-pic24-and-dspic-v2.00-linux-installer.run


```sh
$ chmod u+x peripheral-libraries-for-pic24-and-dspic-v2.00-linux-installer.run
$ sudo ./peripheral-libraries-for-pic24-and-dspic-v2.00-linux-installer.run --help
$ sudo ./peripheral-libraries-for-pic24-and-dspic-v2.00-linux-installer.run --mode text
Do not install the default path: /opt/microchip/xc16
install: /opt/microchip/xc16/v1.25
```




## MPLABX (IDE)

* require:

```sh
# JAVA
$ sudo apt-get install open-jdk-8

# Ubuntu 14.04:
$ sudo apt-get install libc6:i386 libx11-6:i386 libxext6:i386 libstdc++6:i386 libexpat1:i386
```

apt-get install libc6-i386 libx11-6:i386 libasound2:i386 libatk1.0-0:i386 libcairo2:i386 libcups2:i386 libdbus-glib-1-2:i386 libgconf-2-4:i386 libgdk-pixbuf2.0-0:i386 libgtk-3-0:i386 libice6:i386 libncurses5:i386 libsm6:i386 liborbit2:i386 libudev1:i386 libusb-0.1-4:i386 libstdc++6:i386 libxt6:i386 libxtst6:i386 libgnomeui-0:i386 libusb-1.0-0-dev:i386 libcanberra-gtk-module:i386



```sh
$ dpkg --print-foreign-architectures
i386
```


## UART sim

[電子工作室](http://www.picfun.com/mplab/V730/mplab316.html)



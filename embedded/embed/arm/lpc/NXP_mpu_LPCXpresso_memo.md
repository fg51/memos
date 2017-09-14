NXP LPCXpresso
========================================

/usr/local/lpcxpresso/bin/lpcxpresso

## platform

### ubuntu14.04

install 32bit packages in 64bit
```sh
$ sudo dpkg --add-architecture i386
$ sudo apt-get update
$ sudo apt-get install libgtk2.0-0:i386 libpangox-1.0-0:i386 libpangoxft-1.0-0:i386 libidn11:i386 libglu1-mesa:i386 libxtst6:i386 libncurses5:i386
```

$ sudo apt-get install libc6-i386 libx11-6:i386 libxext6:i386 libstdc++6:i386 libexpat1:i386
$ sudo apt-get install software-properties-common   #NOTE: for apt-add-repository
$ sudo apt-add-repository ppa:openjdk-r/ppa
$ sudo apt-get update
$ sudo apt-get install openjdk-8-jdk


[Installing LPCXpresso on Ubuntu 14.04 based distribution | The Red–black tree](https://theredblacktree.wordpress.com/2014/05/17/installing-lpcxpresso-on-ubuntu-14-04-based-distribution/)


L1OU-BQA1-G2KX-CULW-DUJW-FUJY-IQIT-OXJR-CSEV-L1OY-NXLX

### archlinux

http://lerlacher.de/posts/2014-04-12-archlinux-x64-lpcexpresso7.html



## to dfu-util

### require

```bash
$ sudo apt-get install libusb-0.1-4:i386

$ cd /lib/i386-linux-gnu
$ sudo ln -sf libudev.so.1 libudev.so.0
```

## commandline

[Using the command line flash programming tool | www.LPCware.com](https://www.lpcware.com/content/faq/lpcxpresso/command-line-flash-programming)

$ crt_emu_cm_redlink -flash-load-exec "LPC11U68_App.axf" -vendor=NXP -pLPC11U68



[Building projects from the command line | www.LPCware.com](https://www.lpcware.com/content/faq/lpcxpresso/building-projects-command-line)



## CAUTION !!

```bash
$ sudo apt-get install libc6:i386 libusb-dev:i386 uuid-dev:i386 libgtk2.0-0:i386 gtk2-engines-murrine:i386

以下のパッケージは「削除」されます:
  binutils build-essential clang-3.6 debhelper devscripts dkms dpkg-dev g++
  g++-4.8 gcc gcc-4.8 gfortran gfortran-4.8 hardening-includes libtool lintian
  network-manager-pptp network-manager-pptp-gnome pptp-linux tcl-dev
  tcl8.5-dev tcl8.6-dev
以下のパッケージが新たにインストールされます:
  gtk2-engines-murrine:i386 libc6-dev:i386 libpango1.0-0:i386 libusb-dev:i386
  libuuid1:i386 linux-libc-dev:i386 uuid-dev:i386
アップグレード: 0 個、新規インストール: 7 個、削除: 22 個、保留: 0 個。
2,502 kB のアーカイブを取得する必要があります。
この操作後に 161 MB のディスク容量が解放されます。
```

gcc g++ gfortran

sudo apt-get install binutils build-essential clang-3.6 debhelper devscripts dkms dpkg-dev g++-4.8 gcc-4.8 gfortran-4.8 hardening-includes libtool lintian network-manager-pptp network-manager-pptp-gnome pptp-linux tcl-dev tcl8.5-dev tcl8.6-dev




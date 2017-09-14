uname -a
lsmod
lsusb

無線LAN の認識がおかしいので
上記作業の前後の diff を見た方が良い



install for PC
========================================
$ sudo aptitude install build-essential libncurses-dev git git-core
$ sudo aptitude install lib32z1 lib32ncurses5 lib32bz2-1.0  #NOTE: for 32bit arm

#NOTE: for kernel tree
$ git clone https://github.com/raspberrypi/linux

#NOTE: for cross-compile tool
$ git clone https://github.com/raspberrypi/tools


#NOTE: set config
$ cd linux
$ ARCH=arm make bcmrpi_defconfig

configration
    bcmrpi_defconfig            : default config
    bcmrpi_cutdown_defconfig    : down-memory, down-file-size ver
    bcmrpi_emergency_defconfig  : for emergency
    bcmrpi_quick_defconfig      : low-function

build 
----------------------------------------
#NOTE: CAUTION: need 0.5 to 1 hour
$ ARCH=arm CROSS_COMPILE=./tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian/bin/arm-linux-gnueabihf- make -j5

    #NOTE: -j<num of cpu-core>

ref
----------------------------------------
RaspberryPiメモ RaspberryPiクロス開発環境
http://info.ms-n.org/?eid=2

$ sudo aptitude update
$ sudo aptitude upgrade 
$ sudo aptitude clean
$ sudo aptitude install build-essential libncurses5-dev
$ sudo aptitude lib32z1 lib32ncurses5 lib32bz2-1.0
    #NOTE: gitで取得したarmのクロスコンパイラなど開発環境は32ビット版
    そのため32ビット版が実行できるようインストールが必要です。
    32ビット版が実行できない場合、ファイルが存在するにも関わらず”ファイルが無い”といった意味不明なエラーが出ます

$ mkdir raspi
$ cd raspi
$ git clone https://github.com/raspberrypi/linux.git
$ git clone https://github.com/raspberrypi/tools.git

ubuntuのhomeディレクトリにある .profileにPATHを追加します。
$ PATH="$HOME/raspi/tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin:$PATH"

create .config, makefile
----------------------------------------
以下,表記を略す
CROSS_COMPILE = /home/nakamura/raspi/tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin/arm-bcm2708hardfp-linux-gnueabi-

コンフィグレーションfileをRaspberryPiから取ってくる。
$ rsync DNSで引ける名前またはIPアドレス:/proc/config.gz ./
$ cd linux
$ zcat ../config.gz > .config

一応
$ ARCH=arm CROSS_COMPILE= *** make oldconfig
($ ARCH=arm make menuconfig ?)
などをしておきます。
HIFI_BERRY: m ?

makeに引数（ARCH=armなど）を付けても良いのですが、毎回は面倒なのでMakefileに手を加えました。
差分は、

誰か@ubuntu64-VirtualBox:~/raspi/linux$ diff -u Makefile.org Makefile
--- Makefile.org        2014-07-12 16:47:31.672590058 +0900
+++ Makefile    2014-07-12 10:40:53.717043464 +0900
@@ -195,6 +195,11 @@
 ARCH           ?= $(SUBARCH)
 CROSS_COMPILE  ?= $(CONFIG_CROSS_COMPILE:"%"=%)

+ ############## #####################
+ARCH = arm
+CROSS_COMPILE = /home/nakamura/raspi/tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin/arm-bcm2708hardfp-linux-gnueabi-
+ ####################################
+
 # Architecture as present in compile.h
 UTS_MACHINE    := $(ARCH)
 SRCARCH        := $(ARCH)
誰か@ubuntu64-VirtualBox:~/raspi/linux$

これでクロスコンパイルが可能です。

build kernel
----------------------------------------
4coreのマシンでビルドした結果:
kernelのビルド
誰か@ubuntu64-VirtualBox:~/raspi/linux$ time make -j5
---snip---
  IHEX2FW firmware/keyspan_pda/keyspan_pda.fw
  IHEX2FW firmware/keyspan_pda/xircom_pgs.fw

real    59m56.703s
user    127m24.516s
sys     72m35.087s
誰か@ubuntu64-VirtualBox:~/raspi/linux$

build modules
----------------------------------------
$ make modules -j5


RaspberryPiへ転送するための準備
----------------------------------------
$ mkdir tmp
$ make INSTALL_MOD_PATH=./tmp modules_install
RaspberryPiへ転送
$ rsync -avDH tmp/lib RaspberryPiのIPアドレスまたはDNSで引ける名前:/home/誰か/一旦置くディレクトリ名/
$ rsync arch/arm/boot/zImage RaspberryPiのIPアドレスまたはDNSで引ける名前:/home/誰か/一旦置くディレクトリ名/
$ rsync ./zImage RaspberryPiのIPアドレスまたはDNSで引ける名前:/home/誰か/一旦置くディレクトリ名/

転送
----------------------------------------
RaspberryPiのコンソールやsshで接続した端末で次のようにコピーします。
$ sudo cp -rp 一旦置くディレクトリ名/lib /
$ sudo cp -rp 一旦置くディレクトリ名/lib /zImage /boot/kernel.img
$ sudo reboot
再起動後、loginし、確認する。
誰か@raspberrypi ~ $ uname -a
Linux raspberrypi 3.12.24+ #2 PREEMPT Sat Jul 12 14:40:51 JST 2014 armv6l GNU/Linux
誰か@raspberrypi ~ $

本日の遊びはここまでです。




install in /dev/sdb2 of rasbian of SD card
----------------------------------------
cd ~/raspberry/linux/

#NOTE: install modules in sdb2
$ sudo mount /dev/sdb2 /mnt
$ sudo make ARCH=arm INSTALL_MOD_PATH=/mnt/ modules_install
$ sync
$ sudo umount /mnt

#NOTE: install kernel in sdb1
$ sudo umount /mnt
$ sudo mount /dev/sdb1 /mnt
$ sudo cp arch/arm/boot/Image /mnt/kernel.img
$ sync
$ sudo umount /mnt

zImage ? 

build した kernel が反映されたかどうかはどうやって確認するの ???


install with raspberrypi (stand-alone)
========================================
$ git clone https://github.com/raspberrypi/linux



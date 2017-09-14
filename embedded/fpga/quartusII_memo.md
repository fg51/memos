QualtusII
========================================

* CAUTION: DE0 (cycloneIII) is supported via ver.13.1
* CAUTION: DE0 (cycloneIII) is supported via ver.15


## install

[qualtusII in ubuntu](http://labs.beatcraft.com/ja/index.php?DE0-Nano/Synthesijer)


### QualtusII altera

```bash
# 32bit lib
$ apt-get install lib32z1 lib32ncurses5 lib32bz2-1.0
$ bash setup.sh
```


### usb-blaster


```bash
$ lsusb
...
Bus 002 Device 008: ID 09fb:6001 Altera Blaster
$ sudo vi /etc/udev/rules.d/51-usbblaster.rules

SUBSYSTEM=="usb", ENV{DEVTYPE}=="usb_device", ATTR{idVendor}=="09fb", 
ATTR{idProduct}=="6001", MODE="0666", 
NAME="bus/usb/$env{BUSNUM}/$env{DEVNUM}", RUN+="/bin/chmod 0666 %c"

$ sudo udevadm control --reload-rules
```

### run

```bash
$ /home/${user}/altera/15.0/quartus/bin/quartus --64bit
```

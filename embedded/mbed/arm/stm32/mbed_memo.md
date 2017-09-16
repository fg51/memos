mbed stm32
====

mbed is recognised as "USB memory" and "ttyACM0".


```sh
$ dmesg
[  193.179711] usb 2-1.2: new full-speed USB device number 5 using ehci-pci
[  193.340571] cdc_acm 2-1.2:1.2: ttyACM0: USB ACM device
[  193.340902] usbcore: registered new interface driver cdc_acm
[  193.340906] cdc_acm: USB Abstract Control Model driver for USB modems and ISDN adapters
[  193.343146] usb-storage 2-1.2:1.1: USB Mass Storage device detected
[  193.347968] scsi host6: usb-storage 2-1.2:1.1
[  193.348126] usbcore: registered new interface driver usb-storage
[  193.349783] usbcore: registered new interface driver uas
[  194.351500] scsi 6:0:0:0: Direct-Access     MBED     microcontroller  1.0  PQ: 0 ANSI: 2
[  194.354840] sd 6:0:0:0: [sdb] 160 512-byte logical blocks: (81.9 kB/80.0 KiB)
[  194.355564] sd 6:0:0:0: [sdb] Write Protect is off
[  194.355580] sd 6:0:0:0: [sdb] Mode Sense: 03 00 00 00
[  194.356270] sd 6:0:0:0: [sdb] No Caching mode page found
[  194.356283] sd 6:0:0:0: [sdb] Assuming drive cache: write through
[  194.367323]  sdb:
[  194.370257] sd 6:0:0:0: [sdb] Attached SCSI removable disk
```




stlink
====

## install

```bash
$ sudo pacman stlink
```


## update udev rule

49-stlinkv2-1.rules

```
# stm32 nucleo boards, with onboard st/linkv2-1
# ie, STM32F0, STM32F4.
# STM32VL has st/linkv1, which is quite different

- SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
-     MODE:="0666", \
-     SYMLINK+="stlinkv2-1_%n"

+ SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
+     KERNEL!="sd*", KERNEL!="sg*", KERNEL!="tty*", SUBSYSTEM!="bsg", \
+     MODE:="0666", \
+     SYMLINK+="stlinkv2-1_%n"

+ SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
+     KERNEL=="sd*", \
+     MODE:="0666", \
+     SYMLINK+="stlinkv2-1_disk"

+ SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
+     KERNEL=="sg*", \
+     MODE:="0666", \
+     SYMLINK+="stlinkv2-1_raw_scsi"

+ SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
+     KERNEL=="bsg*", \
+     MODE:="0666", \
+     SYMLINK+="stlinkv2-1_block_scsi"

+ SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
+     KERNEL=="tty*", \
+     MODE:="0666", \
+     SYMLINK+="stlinkv2-1_console"

# If you share your linux system with other users, or just don't like the
# idea of write permission for everybody, you can replace MODE:="0666" with
# OWNER:="yourusername" to create the device owned by you, or with
# GROUP:="somegroupname" and mange access using standard unix groups.
```

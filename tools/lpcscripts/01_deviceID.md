
```sh
$ lsusb
Bus 002 Device 010: ID 1fc9:000c NXP Semiconductors 

Bus 002 Device 004: ID 8087:07da Intel Corp. 
Bus 002 Device 003: ID 046d:c52b Logitech, Inc. Unifying Receiver
Bus 002 Device 002: ID 8087:0024 Intel Corp. Integrated Rate Matching Hub
Bus 002 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 001 Device 002: ID 8087:0024 Intel Corp. Integrated Rate Matching Hub
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub

$ boot_lpcscript
Looking for DFU devices with VID 1fc9 PID 000c ...
dfu-util -d 0x1fc9:0x000c -c 0 -i 0 -t 2048 -R -D /usr/local/lpcscrypt/lpcscrypt/scripts/../bin/LPCScrypt_124.bin.hdr
(ignoring exit dfu-util code 1 after successful operation)
Booted LPCScrypt target (0x1fc9:0x000c) with /usr/local/lpcscrypt/lpcscrypt/scripts/../bin/LPCScrypt_124.bin.hdr

$ lsusb
Bus 002 Device 011: ID 1fc9:0083 NXP Semiconductors 

Bus 002 Device 004: ID 8087:07da Intel Corp. 
Bus 002 Device 003: ID 046d:c52b Logitech, Inc. Unifying Receiver
Bus 002 Device 002: ID 8087:0024 Intel Corp. Integrated Rate Matching Hub
Bus 002 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 001 Device 002: ID 8087:0024 Intel Corp. Integrated Rate Matching Hub
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
```

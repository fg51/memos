LPC43xx
========================================


## 1. LPCScrypt

[LPCScrypt | www.LPCware.com](href="https://www.lpcware.com/lpcscrypt)

 ./script/boot_lpcscrypt
./bin/lpcscrypt -h
./bin/lpcscrypt targetinfo


### install CMSIS-DAP

connect the board for DEBUGGER


```sh
# boot scrypt and flush (write) CMSIS-DAP
./script/program_CMSIS
```

install CMSIS-DAP in the flash of the board



## LPC-link2 as DEBUGGER

* JP1: open : DFU (Direct File Upload mode)
       close: boot from ROM
* JP2: short as output the power supply to the other board)


## LPC-link2 as target

* JP1: open


debug-out: CON.j7 <-> target-in: CON.j2


[LPClink2 led blinky](https://www.lpcware.com/content/faq/lpcxpresso/using-lpclink2-as-lpc4370-eval)


1. a

* Emulator family: CMSIS-DAP
* Name: LPC LINK2 CMSIS
* serial number: /dev/hidraw3
* Manufacturer: NXP Semiconductors

2. select JTAG multi core 

* JTAG multi core
* SWD single core


3. select Cortex-M4
* 0 Cortex-M4
* 1 Cortex-M0
* 2 Cortex-M0






LPC-Link2
========================================

[Using an LPC-Link2 as an LPC4370 evaluation board | www.LPCware.com](https://www.lpcware.com/content/faq/lpcxpresso/using-lpclink2-as-lpc4370-eval)

[LPCOpen Software Development Platform (LPC43xx packages) | www.LPCware.com](https://www.lpcware.com/content/nxpfile/lpcopen-software-development-platform-lpc43xx-packages)

[LPCXpresso: sprintfの使い方 | easy labo](http://easylabo.com/2014/08/lpcxpresso/3357/)


## project setting

C/C++ Build > MCU settings
* target: LPC4370
* Flash driver: LPC18\_43\_SPIFI\_1MB\_4KB.cfx

float: FPv2-SP (Soft ABI)

type    Name            Alias   Location    Size
Flash   Flash\_1MB      Flash   0x14000000  0x100000
RAM     RamLoc128       RAM     0x10000000  0x20000
RAM     RamLoc72        RAM2    0x10080000  0x12000

RAM     RamM0Sub16      RAM3    0x18000000  0x4000
RAM     RamM0Sub2       RAM4    0x18004000  0x800



## freertos\_blinky require the LPCOpen-lib

* lpc\_board\_nxp\_lpclink2\_4370
* lpc\_chip\_43xx

* lib\_lpcspifilib




NU gdb (GNU Tools for ARM Embedded Processors) 7.8.0.20150604-cvs
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "--host=i686-linux-gnu --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
0x10001b32 in I2C_SendByte (I2Cx=I2Cx@entry=0x400a1000, databyte=124 '|') at ../src/lpc43xx_i2c.c:185
185		while (!(I2Cx->CONSET & I2C_I2CONSET_SI));


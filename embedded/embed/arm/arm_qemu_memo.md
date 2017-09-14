ARM in QEMU
====

[水銀室 仮想化ハイパーバイザKVMとQEMUを動作させる -openSUSE Linuxデスクトップ環境構築-](http://hp.vector.co.jp/authors/VA022911/tec/suse/kvm_virtmanager.htm)


## install

```sh
#(in ubuntu 12.04)
$ sudo aptitude install qemu qemu-kvm-extras

#(in ubuntu 14.x)
$ sudo aptitude install qemu qemu-system-arm

$ sudo aptitude install gcc-arm-linux-gnueabi libc6-armel-armhf-cross
```

* qemu: QEMU emulator
* qemu-kvm-extras: emulator for ARM
* gcc-arm-linux-gnueabi: cross compiler
* libc6-armel-armhf-cross: library


## build

```sh
$ arm-linux-gnueabi-gcc -o <dest>.elf

$ arm-linux-gnueabi-gcc -mcpu=arm9 -marm -o <dest>.elf
```

* -mcpu=arm9: executable for arm9
* -marm: output arm-command

```sh
$ file hello.elf
hello.elf: ELF 32-bit LSB  executable, ARM, EABI5 version 1 (SYSV),
dynamically linked (uses shared libs), for GNU/Linux 2.6.32, 
BuildID[sha1]=f9ce36f8c97ae77b6704d75cbb4595db42f2c0bf, not stripped

$ qemu-arm -L /usr/arm-linux-gnueabi/ <dest>.elf
```

means: file type is **ARM, EABI5 version 1**
ade

"/usr/arm-linux-gnueabi/" require "gcc-arm-linux-gnueabi"


## format

```cpp
__asm__ (
    "assembly code \n\t #comment"   //comment
    : operand for outout
    : operand for input
    : override register
    );
```


### operand for output

: [macro1] "operand rule" (arg1), [macro2] "operand rule" (arg2)

operand rule
:r: usually register (r0 to r15)
:I: Immediate (literal)
:M: ** 2
:m: memory adress

:=: write only operand
:+: write and read operand
:&: output only register



### override register

asm 実行前に，指定した register の値をスタックに退避する。asm 終了後，元に戻す。
これを指定しないとregister の値を壊す。(compile error 出ない。)


## ARM assembly

function-call 
:bl: <func>
arg1 is reg: r0
arg2 is reg: r1
return-value is reg: r0
return is bx lr

:b: goto ?
:ble: Branch Less than or Equal ?


## WORD

1 word = 32 bit (4 Byte)
Opecode (16bit) + operand (8bit) + operand (8bit)


## CPSR

Current Program Status Register
32bit (8bit * 4)

flag value (caluc result)   cpsr[24 ... 31]
status value                cpsr[16 ... 23]
expand value                cpsr[ 8 ... 15]
control value               cpsr[ 0 ...  7]

in flag, 
cpsr[28] is V overflow
cpsr[29] is C carry
cpsr[30] is Z zero
cpsr[31] is N negative

in control,
cpsr[7] is IRQ mask
cpsr[6] is FIQ mask
cpsr[5] is Thumb state


## MOVE

MOVE{S} Rd, <Operand2> NZC

MOV  a, b   @ a<-b
MOVN a, b   @ bit reverse

if use over8bit value, opecode: LDR

MOV <dest>, <org>

MOVN r1, #10    @r1 = 0xfffffff5 (~10)


MOV{S} is applied register: cpsr

:N: Negative
:Z: Zero
:C: Carry
:V: oVerflow


## ADD

ADD{S} Rd, Rn, <Operand2>
ADD r2, r0, r1  @ 0x2468 <- 0x1234 + 0x1234
ADC r2, r0, r1  @ 0x2469 <- 0x1234 + 0x1234 + carry-flag


## SUB

SUBtract
SUB r2, r0, r1       0x0ccd <- 0x1234 - 0x0567
SUB r2, r0, #0x10    0x1224 <- 0x1234 - 0x0010

SuBtract with Carry
SBC r2, r0, r1       0xccd <- 0x1234 - 0x567 - NOT(carry) (at carry==1)

Reverse SuBtract
RSB r2, r0, r1       0xfffff333 <- 0x0567 - 0x1234
RSC r2, r0, r1       0xfffff333 <- 0x0567 - 0x1234 - NOT(carry) (at carry==1)


## LDR

immediate value is 8bit

for over 8bit, use LDR


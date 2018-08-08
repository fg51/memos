[STM32のバイナリをQEMUで動かしてみた &#8211; 楽しくやろう。](https://blog.boochow.com/article/456638901.html)

$ qemu-system-gnuarmeclipse \
  --verbose --verbose \
  --board <board name> \
  --mcu <mcu name> \
  -d unimp,guest_errors
  --gdb tcp::1234 \
  --nographic --image <prog name>.elf \
  --semihosting-config enable=on,target=native \
  --semihosting-cmdline <prog name>



start gdb

```sh
$ arm-none-eabi-gdb blinky.elf
```

connect QEMU's gdb server

```sh
(gdb) $ tar remote :1234
```

example

```sh
$ ./bin/qemu-system-gnuarmeclipse \
  --verbose --verbose \
  --board NUCLEO-F103RB \
  --mcu STM32F103RB \
  -d unimp,guest_errors
  --gdb tcp::1234 \
  --nographic --image blinky.elf \
  --semihosting-config enable=on,target=native \
  --semihosting-cmdline blinky
```

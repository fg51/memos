mbed-cli
====

[mbed OS 5 の CLI 開発環境を Linux 上に構築する - Qiita](http://qiita.com/hotchpotch/items/4d87c13b97d236db5b4f)

## install

```sh
$ pip2 install mbed-cli
$ pip3 install mbed-cli  # python3.6 or later, and mbedOS 5.9 or later
```

## sample import

```sh
$ mbed import https://developer.mbed.org/teams/mbed-os-examples/code/mbed-os-example-blinky
$ cd mbed-os-example-blinky
$ mbed detect

$ ls .
.gitignore
.hg/
.mbed
img/
main.cpp
mbed-os/
mbed-os.lib
mbed_settings.py
README.md
```

```sh
$ git clone https://developer.mbed.org/teams/mbed-os-examples/code/mbed-os-example-blinky
$ cd mbed-os-example-blinky
$ mbed deploy   # NOTE: download src
$ mbed detect

$ ls .
.gitignore
.hg/
.mbed
img/
main.cpp
mbed-os/
mbed-os.lib
mbed_settings.py
README.md
```

* mbed_settings.py: config file

GCC

```sh
$ mbed target <target cpu>
$ mbed toolchain GCC_ARM
$ mbed compile
```



mbed import http

```sh
$ mbed import https://developer.mbed.org/teams/mbed/code/mbed_blinky/
$ mbed deploy   # for library
```

$ mbed import https://developer.mbed.org/teams/mbed/code/mbed_blinky/

mbed OS: https://github.com/ARMmbed/mbed-os-example-blinky


## create project

```sh
$ mbed new <proj>
```

## mount mbed

```sh
$ sudo mount -t vfat /dev/sdb /mnt/usb
$ mbed detect
$ mbedls
```

## serial communication

```sh
$ minicom -s
```


## build

```sh
$ mbed config --global GCC_ARM "toolchain/install/bin/"
$ cat mbed_settings.py
  # GCC ARM
- #GCC_ARM_PATH = ""
+ GCC_ARM_PATH = "/opt/gcc-arm-none-eabi-5_3_2016q1/bin"
```

```sh
$ mbed compile -m NUCLEO_F446RE
```

mbed compile --profile <flag>

flag: Develop, Debug, Release

[docs.mbed.com](https://docs.mbed.com/docs/mbed-os-handbook/en/latest/dev_tools/build_profile)



### build config

```sh
$ mbed comple --profile <filepath.json>
```

default profile: targets.json


```mbed-os/targets.json
{
    "Target": {
        "core": null,
        "default_toolchain": "ARM",
        "supported_toolchains": null,
        "extra_labels": [],
        "is_disk_virtual": false,
        "macros": [],
        "device_has": [],
        "features": [],
        "detect_code": [],
        "public": false,
        "default_lib": "std",
        "bootloader_supported": false
    },

    "FAMILY_STM32": {
        "inherits": ["Target"],
        "public": false,
        "extra_labels": ["STM"],
        "supported_toolchains": ["ARM", "uARM", "IAR", "GCC_ARM"],
        "macros": ["TRANSACTION_QUEUE_SIZE_SPI=2"],
        "device_has": ["ANALOGIN", "I2C", "I2CSLAVE", "I2C_ASYNCH", "INTERRUPTIN", "PORTIN", "PORTINOUT", "PORTOUT", "PWMOUT", "RTC", "SERIAL", "SLEEP", "SPI", "SPISLAVE", "SPI_ASYNCH", "STDIO_MESSAGES"]
    },

    "NUCLEO_F302R8": {
        "inherits": ["FAMILY_STM32"],
        "supported_form_factors": ["ARDUINO", "MORPHO"],
        "core": "Cortex-M4F",
        "extra_labels_add": ["STM32F3", "STM32F302x8", "STM32F302R8"],
        "detect_code": ["0705"],
        "device_has_add": ["ANALOGOUT", "CAN", "LOWPOWERTIMER", "SERIAL_ASYNCH", "SERIAL_FC"],
        "default_lib": "small",
        "release_versions": ["2"],
        "device_name": "STM32F302R8"
    },


    "NUCLEO_F401RE": {
        "inherits": ["FAMILY_STM32"],
        "supported_form_factors": ["ARDUINO", "MORPHO"],
        "core": "Cortex-M4F",
        "extra_labels_add": ["STM32F4", "STM32F401xE", "STM32F401RE"],
        "config": {
            "clock_source": {
                "help": "Mask value : USE_PLL_HSE_EXTC | USE_PLL_HSE_XTAL (need HW patch) | USE_PLL_HSI",
                "value": "USE_PLL_HSE_EXTC|USE_PLL_HSE_XTAL|USE_PLL_HSI",
                "macro_name": "CLOCK_SOURCE"
            }
        },
        "detect_code": ["0720"],
        "macros_add": ["USB_STM_HAL", "USBHOST_OTHER"],
        "device_has_add": ["ERROR_RED", "SERIAL_ASYNCH", "SERIAL_FC"],
        "release_versions": ["2", "5"],
        "device_name": "STM32F401RE"
    },
    "NUCLEO_F446RE": {
        "inherits": ["FAMILY_STM32"],
        "supported_form_factors": ["ARDUINO", "MORPHO"],
        "core": "Cortex-M4F",
        "extra_labels_add": ["STM32F4", "STM32F446xE", "STM32F446RE"],
        "config": {
            "clock_source": {
                "help": "Mask value : USE_PLL_HSE_EXTC | USE_PLL_HSE_XTAL (need HW patch) | USE_PLL_HSI",
                "value": "USE_PLL_HSE_EXTC|USE_PLL_HSE_XTAL|USE_PLL_HSI",
                "macro_name": "CLOCK_SOURCE"
            }
        },
        "detect_code": ["0777"],
        "macros_add": ["USB_STM_HAL", "USBHOST_OTHER"],
        "device_has_add": ["ANALOGOUT", "CAN", "ERROR_RED", "LOWPOWERTIMER", "SERIAL_ASYNCH", "SERIAL_FC"],
        "release_versions": ["2", "5"],
        "device_name": "STM32F446RE"
    },

```

```mbed-os/tools/profiles/debug.json
{
    "GCC_ARM": {
        "common": ["-c", "-Wall", "-Wextra",
                   "-Wno-unused-parameter", "-Wno-missing-field-initializers",
                   "-fmessage-length=0", "-fno-exceptions", "-fno-builtin",
                   "-ffunction-sections", "-fdata-sections", "-funsigned-char",
                   "-MMD", "-fno-delete-null-pointer-checks",
                   "-fomit-frame-pointer", "-O0", "-g3", "-DMBED_DEBUG",
                   "-DMBED_TRAP_ERRORS_ENABLED=1"],
        "asm": ["-x", "assembler-with-cpp"],
        "c": ["-std=gnu99"],
        "cxx": ["-std=gnu++98", "-fno-rtti", "-Wvla"],
        "ld": ["-Wl,--gc-sections", "-Wl,--wrap,main", "-Wl,--wrap,_malloc_r",
               "-Wl,--wrap,_free_r", "-Wl,--wrap,_realloc_r", "-Wl,--wrap,_memalign_r",
               "-Wl,--wrap,_calloc_r", "-Wl,--wrap,exit", "-Wl,--wrap,atexit",
               "-Wl,-n"]
    },
```

```mbed-os/tools/profiles/release.json
{
    "GCC_ARM": {
        "common": ["-c", "-Wall", "-Wextra",
                   "-Wno-unused-parameter", "-Wno-missing-field-initializers",
                   "-fmessage-length=0", "-fno-exceptions", "-fno-builtin",
                   "-ffunction-sections", "-fdata-sections", "-funsigned-char",
                   "-MMD", "-fno-delete-null-pointer-checks",
                   "-fomit-frame-pointer", "-Os", "-DNDEBUG"],
        "asm": ["-x", "assembler-with-cpp"],
        "c": ["-std=gnu99"],
        "cxx": ["-std=gnu++98", "-fno-rtti", "-Wvla"],
        "ld": ["-Wl,--gc-sections", "-Wl,--wrap,main", "-Wl,--wrap,_malloc_r",
               "-Wl,--wrap,_free_r", "-Wl,--wrap,_realloc_r", "-Wl,--wrap,_memalign_r",
               "-Wl,--wrap,_calloc_r", "-Wl,--wrap,exit", "-Wl,--wrap,atexit",
               "-Wl,-n"]
    },
    "ARM": {
        "common": ["-c", "--gnu", "-Ospace", "--split_sections",
                   "--apcs=interwork", "--brief_diagnostics", "--restrict",
                   "--multibyte_chars", "-O3", "-DNDEBUG"],
        "asm": [],
        "c": ["--md", "--no_depend_system_headers", "--c99", "-D__ASSERT_MSG"],
        "cxx": ["--cpp", "--no_rtti", "--no_vla"],
        "ld": []
    },
    "uARM": {
        "common": ["-c", "--gnu", "-Ospace", "--split_sections",
                   "--apcs=interwork", "--brief_diagnostics", "--restrict",
                   "--multibyte_chars", "-O3", "-D__MICROLIB",
                   "--library_type=microlib", "-DMBED_RTOS_SINGLE_THREAD", "-DNDEBUG"],
        "asm": [],
        "c": ["--md", "--no_depend_system_headers", "--c99", "-D__ASSERT_MSG"],
        "cxx": ["--cpp", "--no_rtti", "--no_vla"],
        "ld": ["--library_type=microlib"]
    },
    "IAR": {
        "common": [
            "--no_wrap_diagnostics", "-e",
            "--diag_suppress=Pa050,Pa084,Pa093,Pa082", "-Ohz", "-DNDEBUG"],
        "asm": [],
        "c": ["--vla"],
        "cxx": ["--guard_calls", "--no_static_destruction"],
        "ld": ["--skip_dynamic_initialization", "--threaded_lib"]
    }
}
```

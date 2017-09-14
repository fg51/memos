mbed-cli
====

[mbed OS 5 の CLI 開発環境を Linux 上に構築する - Qiita](http://qiita.com/hotchpotch/items/4d87c13b97d236db5b4f)

## install

```sh
$ pip2 install mbed-cli
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

* mbed_settings.py: config file

GCC

```sh
$ mbed target <target cpu>
$ mbed toolchain GCC_ARM
$ mbed compile
```





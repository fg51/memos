archlinux in wsl
====

# 0. windows
turn on/off Windows features

[X] Windows subsystem for Linux 
[X] Virtural Machine Platform

```sh
$ wsl --update
$ wsl --version
```

# 1. tarball

in any linux

```sh
$ wget ${TAR BALL URL}
$ sudo tar xzvf ${any.tar.gz}
$ sudo vim root.x86_64/etc/pacman.d/mirrorlist
$ sudo tar czvf ${any1}.tar.gz *
```

# 2. import to wsl

```sh
$ mkdir \p C:\wsl\arch
$ wsl --import ${WSL NAME} C:\wsl\arch ${any1}.tar.gz
$ wsl --list
```

# 3.

```sh
$ pacman-key --init
$ pacman-key --populate archlinux

$ pacman -Syyu base base-devel git vim wget reflector zsh
$ pacman -Syy
$ export EDITOR="(which vim)"

$ reflector --country Japan --age 12 --protocol https --sort rate --save /etc/pacman.d/mirrorlist

# set passwd for root
$ passwd

# add user
$ useradd -m -G wheel -s /bin/zsh -d /home/{USERNAME} {USERNAME}
$ passwd {USERNAME}

# add sudoer for wheel group.
$ sudoedit /etc/sudoers
# comment out the (`%wheel ALL=(ALL:ALL) ALL` line
```

## locale
```sh
$ locale-gen
$ echo LANG=en_US.UTF-8 > /etc/locale.conf
$ export LANG=en_US.UTF-8
```

## default user

```@/etc/wsl.conf
[user]
default={USERNAME}

[boot]
systemd = true

[wsl2]
localhostForwarding = true
memory = 8GB
```



```@/etc/wsl.conf
[interop]
appendWindowsPath = false
```

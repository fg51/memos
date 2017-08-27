haskell stack
====


## install

### Archlinux


```/etc/pacman.conf
+ [haskell- core]
+ Server = http://xsounds.org/~haskell/core/$arch

[extra]
Include = /etc/pacman.d/mirrorlist
```

```bash
$ sudo pacman -Syu
$ sudo pacman -S haskell-stack
# $ sudo pacman -S stack ghc-static
```

```bash
$ stack config set system-ghc --global true
$ cat .stack/config/stack.yaml
```

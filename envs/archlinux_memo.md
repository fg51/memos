archlinux
====


## pacaur

aur package manager

### install


```bash
$ gpg --recv-keys 487EACC08557AD082088DABA1EB2638FF56C0C53
$ git clone --depth=1 "https://aur.archlinux.org/cower.git"
$ cd cower
$ makepkg -si
$ cd ..
```

#### install pacaur

```bash
$ cower -d pacaur
$ cd pacaur
$ makepkg -si
```

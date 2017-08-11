fcitx-mozc
====


## install

```bash
$ sudo pacman -S fcitx fcitx-mozc fcitx-configtool fcitx-im
```

## setup

in .xprofile for lightDM (desktop manager)
in .xinitrc for slim (desktop manager)

```
export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export XMODIFIERS="@im=fcitx"
export DefaultIMModule=fcitx
fcitx-autostart
```


in .config/i3/config

```
exec --no-startup-id fcitx
```

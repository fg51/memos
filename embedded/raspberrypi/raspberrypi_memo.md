raspberry pi
====

[Raspberry Pi - Teach, Learn, and Make with Raspberry Pi](https://www.raspberrypi.org)


## install raspbian

umount SD-card

```sh
$ sudo umount /dev/sdb
```


## format ?

```sh
$ sudo parted /dev/sdb
```

$ sudo gparted /dev/sdb


### install

```sh
$ sudo dd bs=4M if=2017-09-07-raspbian-stretch-lite.img of=/dev/sdb
$ sync
```

with progress

```sh
$ sudo dd bs=4M if=2017-09-07-raspbian-stretch-lite.img of=/dev/sdb status=progress conv=fsync
```


### setup ssh (headless install)

[Raspberry Pi zero W のヘッドレスインストール（キーボードやマウスなしでRaspbianをインストールする方法） &#8211; ラズパイダ](https://raspida.com/headless-install2pi0w)


```sh
$ fdisk -l 2017-02-16-raspbian-jessie-lite.img
Disk 2017-02-16-raspbian-jessie-lite.img: 1,3 GiB, 1437597696 bytes, 2807808 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0xdbcc7ab3

Device                               Boot  Start     End Sectors  Size Id Type
2017-02-16-raspbian-jessie-lite.img1        8192  137215  129024   63M  c W95 FAT32 (LBA)
2017-02-16-raspbian-jessie-lite.img2      137216 2807807 2670592  1,3G 83 Linux
```

* in order to mount the boot partition we need to know its start offset in bytes, in our case that would be 512 * 8192 = 4194304


### create a temporary folder to mount Raspbian’s boot partition

```sh
$ mkdir mnt
```

### mount Raspbian’s boot partition

512 * 8192 = 4194304

```sh
$ sudo mount -v -o offset=4194304 -t vfat 2017-02-16-raspbian-jessie-lite.img /mnt/boot
```

### create ssh file to tell Raspbian to enable the ssh daemon by default

```sh
$ sudo touch /mnt/boot/ssh
```


[WPA_SUPPLICANT.CONF(5)_fileformat](http://www.yosbits.com/opensonar/rest/man/freebsd/man/ja/man5/wpa_supplicant.conf.5.html)


```sh
$ wpa_passphrase "ssid" "pass(not coded)" > wpa_supplicant.conf
$ sudo cp wpa_supplicant.conf /etc/wpa_supplicant/wpa_supplicant.conf
$ sudo cat /mnt/boot/wpa_supplicant.conf
crtl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev  # socket path, group name
update_config=1
country=JP
network={
    ssid="WIFI_SSID"
    psk=WIFI_PASSWORD
    proto=RSN   # WPA (IEEE 802.11i/D3.0), RSN (IEEE 802.11i)  default: RSN
    key_mgmt=WPA-PSK  # default: WPA-PSK WPA-EAP
    auth_alg=OPEN
    pairwise=CCMP
    scan_ssid=1  # scan stealth
}
```

```sh
network={
ssid=”myhome_ssid”　# need ””で括る
#psk=”myhome_passwd” # delete it
psk=foobar  # coded password
```


### unmount the partition again

```sh
$ sudo umount mnt
```

* copy the image file to an SD card
* boot your Raspberry Pi, the ssh daemon should be enabled by default


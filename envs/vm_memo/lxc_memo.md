LXC LINUX Containers
========================================

[Linux Containers](https://linuxcontainers.org/ja/)

# install
```bash
$ echo "deb http://ppa.launchpad.net/ubuntu-lxc/lxc-lts/ubuntu trusty main" | sudo tee /etc/apt/sources.list.d/lxc_ltc.list
$ echo "deb-src http://ppa.launchpad.net/ubuntu-lxc/lxc-lts/ubuntu trusty main" | sudo tee /etc/apt/sources.list.d/lxc_ltc.list
```

http://keyserver.ubuntu.com:11371/pks/lookup?op=get&search=0xD5495F657635B973

-----BEGIN PGP PUBLIC KEY BLOCK-----
Version: SKS 1.1.5
Comment: Hostname: keyserver.ubuntu.com

mI0ETiyyWwEEAOOoug44FpzM7z818+73RQKpgoQrTmSRirwQ9FPbxj42gRDhzT+pzTBlyo1q
gwpqzhTY5wYWc9jQBWxX3YEdNM42aBSBlhy/aC8zj6dqZMcRP0/6ImMy+dgXM59QiGVw8CwD
54vmXpgTZy7iOzRo9f7rl50MsbfIbawx2wQiITuzABEBAAG0IUxhdW5jaHBhZCBQUEEgZm9y
IFVidW50dSBMWEMgdGVhbYi4BBMBAgAiBQJOLLJbAhsDBgsJCAcDAgYVCAIJCgsEFgIDAQIe
AQIXgAAKCRDVSV9ldjW5cxl7BADMLDQfq90SH8tpGN3+8ZrtGHhe+Ox5aBj83bvgzL0ynQ+s
pG33VD/P+ITMALE2+QIWgl0XfeWmKAbr0m0mdoudfOSxRKMiq4nfSrnC6B+pmOQ0xAbKPQk2
3eGO+j16HHn/8SnVLsKKj4rfMSn8K/p1rcfRNugAIii68lnYuooz8g==
=hPvA
-----END PGP PUBLIC KEY BLOCK-----


http://keyserver.ubuntu.com:11371/pks/lookup?fingerprint=on&op=index&search=0x93763AC528C8C52568951BE0D5495F657635B973



## usage

### create container with template

```bash
$ ls /usr/share/lxc/templates/
lxc-alpine     lxc-centos    lxc-fedora        lxc-oracle  lxc-ubuntu-cloud
lxc-altlinux   lxc-cirros    lxc-gentoo        lxc-plamo
lxc-archlinux  lxc-debian    lxc-openmandriva  lxc-sshd
lxc-busybox    lxc-download  lxc-opensuse      lxc-ubuntu

$ sudo lxc-create -t <template name> -n <dest name>
$ sudo lxc-create -t ubuntu -n ubuntu-test01
```


 the default user is "ubuntu" with password "ubuntu"


```bash
$ sudo lxc-start -n ubuntu_nxp_74 -d
```

* -d: with daemon
if not using "-d", console is occupied via lxc-console




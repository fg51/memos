
[vagrant + ansible] (http://qiita.com/pengin/items/44dca79a49ccc8263062)


VIRTUALBOX
========================================

2015.08.17 

## install

[oracle virtualbox](https://www.virtualbox.org/wiki/Linux_Downloads)

```bash
# add repogitory
$ echo "deb http://download.virtualbox.org/virtualbox/debian <oneiric> contrib" | sudo tee /etc/apt/sources.list.d/oracle_vbox.list

# for ubuntu 14.04 trusty
$ echo "deb http://download.virtualbox.org/virtualbox/debian trusty contrib" | sudo tee /etc/apt/sources.list.d/oracle_vbox.list

# add key
$ wget -q http://download.virtualbox.org/virtualbox/debian/oracle_vbox.asc -O- | sudo apt-key add -

# check finger key
$ sudo apt-get finger

$ sudo apt-get update
$ sudo apt-get install virtualbox-5.0
```


Vagrant
========================================

[old box](data: http://www.vagrantbox.es/)
ref: http://blog.papix.net/entry/2013/04/27/141608


## install

[vagrant download](https://www.vagrantup.com/downloads.html)
[dpkg memo](http://qiita.com/Itomaki/items/9a6a314a853cdcd00f80)

```bash
$ sudo apt-get install dpkg
$ wget http://*****/vagant_XXXX.deb
$ sudo dpkg -i Vagrant_1.7.2_x86_64.deb
```

## plugin install

```bash
$ vagrant plugin install vagrant-global-status

# usage
$ vagrant global-status -a
```


## add box

[vagrantのboxをvagrant cloudからもらってくる - わすれっぽいきみえ](http://kimikimi714.hatenablog.com/entry/2014/04/05/vagrant%E3%81%AEbox%E3%82%92vagrant_cloud%E3%81%8B%E3%82%89%E3%82%82%E3%82%89%E3%81%A3%E3%81%A6%E3%81%8F%E3%82%8B)

[vagrant cloud](https://atlas.hashicorp.com/boxes/search?utm_source=vagrantcloud.com&vagrantcloud=1)

```bash
$ vagrant login
username:
password:

$ vagrant box add ubuntu/trusty64
$ vagrant box add terrywang/archlinux
```
```



add box (old)
========================================
.. sourcecode:: sh
    $ vagrant box add <dest name> <org box file or url>
    $ vagrant box list
    $ vagrant init <dest name>


### for example CentOS7

```bash
$ mkdir -p ~/myWork/vagrantData/centos7
$ cd ~/myWork/vagrantData/centos7
$ vagrant box add centos7 https://f0fff3908f081cb6461b407be80daf97f07ac418.googledrive.com/host/0BwtuV7VyVTSkUG1PM3pCeDJ4dVE/centos7.box
$ vagrant init centos7
```

### for example ubuntu 14.04

```bash
$ mkdir -p ~/myWork/vagrantData/centos7
$ cd ~/myWork/vagrantData/centos7
$ vagrant box add ubuntu1404 https://github.com/kraksoft/vagrant-box-ubuntu/releases/download/14.04/ubuntu-14.04-amd64.box
$ vagrant init ubuntu1404
```

usage
========================================

```bash
$ pwd
centos7 #vagrantfile in the directory
$ vagrant up
$ vagrant ssh
$ su -
# yum update
# yum upgrade -y
```


:user: root
:pass: vagrant
:ssh key: at ~/.vagrant.d/insecure_private_key


halt
========================================

```bash
$ vagrant halt  # stop
```

destory
========================================

```bash
$ vagrant destory
```


create box / rebuild box
========================================

```bash
(vagrant)$ sudo yum udate -y

#NOTE: delete the NIC map
(vagrant)$ sudo ln -s -f /dev/null /etc/udev/rules.d/70-persistent-net.rules
(vagrant)$ exit
$ vagrant halt

$ vagrant package --output <NAME>
$ ls | grep box$
<NAME>.box

#NOTE: useage
$ vagrant box add mycentos64 package.box
```


vbox error
========================================


Failed to mount folders in Linux guest. This is usually because
the "vboxsf" file system is not available. Please verify that
the guest additions are properly installed in the guest and
can work properly. The command attempted was:

mount -t vboxsf -o uid=`id -u vagrant`,gid=`getent group apache | cut -d: -f3`,dmode=777,fmode=777 /vagrant /vagrant
mount -t vboxsf -o uid=`id -u vagrant`,gid=`id -g apache`,dmode=777,fmode=777 /vagrant /vagrant

.. sourcecode:: sh
    # need the rebuild
    $ vagrant ssh
    $ sudo /etc/init.d/vboxadd setup
    $ vagrant halt
    $ vagrant up

.. sourcecode:: sh
    $ vagrant plugin install vagrant-vbguest



memory + cpu
in Vagrantfile

.. sourcecode:: sh
  config.vm.provider "virtualbox" do |vb|
  #   # Display the VirtualBox GUI when booting the machine
  #   vb.gui = true
  #
  #   # Customize the amount of memory on the VM:
    cpus = `nproc`.to_i
    vb.cpus = cpus
    vb.memory = "2048"
  end


GUI
========================================

use vagrant with gui

.. sourcecode:: sh
    config.vm.provider :virtualbox do |vb|
        #Display the VirtualBox GUI when booting the machine
        vb.gui = true

        vb.customize ["modifyvm", :id, "--memory", "2048"]
        vb.customize ["modifyvm", :id, "--vram"  , "128"]
        vb.customize ["modifyvm", :id, "--accelerate3d", "on"]


# gui package

sudo apt-get install software-properties-common python-software-properties
sudo add-apt-repository ppa:gnome3-team/gnome3
sudo apt-get update

sudo apt-get install -y xfce4
sudo apt-get install -y gnome-shell ubuntu-desktop gdm
sudo dpkg-reconfigure gdm


virtualbox to vagrant box
========================================


audio OFF

USB controller OFF

network
NAT
Intel PRO/1000 MT Desktop
cable connect ON

network > 高度 > port-fowarding

name    protcol host        port    guest       port
SSH     TCP     127.0.0.1   2222    10.0.2.15   22

```bash
$ sudo pacman -S openssh
$ vi /etc/ssh/ssh_config
- UseDNS yes
+ UseDNS no
$ sudo systemctl start sshd.service
$ sudo systemctl enable sshd.service

$ pacman -S virtualbox-guest-utils

$ su - vagrant
$ mkdir ~/.ssh
$ chmod 700 ~/.ssh

$ ssh-keygen -t rsa -f ~/.ssh/vagrant
```

[Archlinux の Vagrant Box (virtualBox) を 一から作る](http://10sr.hateblo.jp/entry/2014/08/19/232011)

[Arch LinuxのVagrant boxを作成する. - Qiita](http://qiita.com/haburibe/items/38d14a00c2fce573e10f)



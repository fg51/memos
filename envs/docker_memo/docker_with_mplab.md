
```bash
$ xhost +local:root

$ docker run -it -e DISPLAY=$DISPLAY -v=/tmp/.X11-unix:/tmp/.X11-unix -v ~/.Xauthority:/root/.Xauthority jiita/ubuntu_xc16 mplab_ide

$ docker run -it --name mplabx --privileged \
    -v /dev/bus/usb:/dev/bus/usb \
    -v /tmp/.X11-unix:/tmp/.X111-unix \
    -v ~/work/mplab/projects:/root/work/mplab/projects \
    -e DISPLAY = $DISPLAY \
    jiita/ubuntu_xc16 mplab_ide
```

$ sudo dpkg --add-architecture i386

```bash@ubuntu14.04
#required
$ sudo apt-get update
$ sudo apt-get install libc6-i386 libx11-6:i386 libxext6:i386 libstdc++6:i386 libexpat1:i386
$ sudo apt-get install software-properties-common   #NOTE: for apt-add-repository
$ sudo apt-add-repository ppa:openjdk-r/ppa
$ sudo apt-get update
$ sudo apt-get install openjdk-8-jdk
```

docker commit 

#!/usr/bin/env bash

echo "start docker with mplabx"
#xhost +local:root

docker run -it --privileged -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v ~/.Xauthority:/root/.Xauthority \
    -v /dev/bus/usb:/dev/bus/usb \
    -v ~/Dropbox/embedded/dspic/motorSamples/MPLABXProjects:/root/MPLABXProjects\
    jiita/lpcxpr:v326 bash

#~/Dropbox/embedded/dspic/motorSamples/feat_init_hard

ref
----------------------------------------
http://www.raspberrypi.org/documentation/installation/installing-images/linux.md

install 1
----------------------------------------
$ df -h
$ umount /dev/sdb1
$ sudo dd bs=4M if={raspbian.img} of=/dev/{SD card}
($ sudo dd bs=4M if=2014-09-09-wheezy-raspbian.img of=/dev/sdb)
(or $ sudo dcfldd bs=4M if={raspbian.img} of=/dev/{SD card})
$ sync

remove the SD-card and set the SD-card in the raspberry-pi

install 2
----------------------------------------
($ sudo raspi-config)
Expand Filesystem: on
internationalistation option:
    change locale: 
        cansel: en_GB.UTF-8
        set   : en_US.UTF-8, ja_JP.UTF-8 (default: en_US.UTF-8)
    timezone: asia > tokyo
    keyboard:
        Generic 105-key(intl) PC > japanese > the default for the keyboard layout > No compose key > no
ssh: on


Avahi
========================================
$ sudo apt-get install avahi-daemon
$ sudo /etc/init.d/avahi-daemon status

connect raspberry pi with avahi, zeroconf, ssh
========================================
$ ssh raspi.local


$ sudo rpi-update
$ sudo apt-get update
$ sudo apt-get upgrade
$ uname -a


wxpython
========================================

install
----------------------------------------
$ sudo apt-get install -y python-wxgtk2.8 python-wxtools wx2.8-i18n




piTFT
========================================

install
----------------------------------------
$ wget http://adafruit-download.s3.amazonaws.com/libraspberrypi-bin-adafruit.deb
$ wget http://adafruit-download.s3.amazonaws.com/libraspberrypi-dev-adafruit.deb
$ wget http://adafruit-download.s3.amazonaws.com/libraspberrypi-doc-adafruit.deb
$ wget http://adafruit-download.s3.amazonaws.com/libraspberrypi0-adafruit.deb
$ wget http://adafruit-download.s3.amazonaws.com/raspberrypi-bootloader-adafruit-20140917-1.deb

$ sudo dpkg -i -B \*.deb

$ sudo mv /usr/share/X11/xorg.conf.d/99-fbturbo.conf ~/bak

$ sudo reboot

$ sudo modprobe spi-bcm2708
$ sudo modprobe fbtft_device name=adafruitts
$ rotate=90
$ export FRAMEBUFFER=/dev/fb1
$ startx

module auto-loading
----------------------------------------
$ sudo vim /etc/modules
    #NOTE: add two line
    spi-bcm2708
    fbtft_device

$ sudo vim /etc/modprobe.d/adafruit.conf
    #NOTE: add following line
    options fbtft_device name=adafruitrt28 rotate=90 
    frequency=32000000

    """
    The rotate= variable tells the driver to rotate the screen 0 90 180 or 270 degrees.
    0 is portrait, with the bottom near the "Adafruit Logo"
    90 is landscape, with the bottom of the screen near the buttons.
    180 is portrait, with the top near the "Adafruit Logo"
    270 is landscape, with the top of the screen near the buttons.
    You can change this file with nano and reboot to make the change stick.
    
    The frequency= variable tells the driver how to fast to drive the display. 32MHz (32000000) is a pretty nice 20 FPS rate but if your screen is acting funny, try taking it down to 16MHz (16000000)
    
    The screenshot below shows "name=adafruitts" but its better to use "name=adafruitrt28" since we now have multiple types of touchscreens, so its good to be super clear we're using the Adafruit Resistive Touch 2.8" (adafruitrt28) driver
    """

$ sudo reboot
    
$ sudo mkdir /etc/X11/xorg.conf.d
$ sudo vim /etc/X11/xorg.conf.d/99-calibration.conf

    Section "InputClass"
            Identifier      "calibration"
            MatchProduct    "stmpe-ts"
            Option  "Calibration"   "3800 200 200 3800"
            Option  "SwapAxes"      "1"
    EndSection


$ FRAMEBUFFER=/dev/fb1 startx
# Ctrl+C to quit X


$ sudo vim ~/.profile
    #NOTE: add follow line
    export FRAMEBUFFER=/dev/fb1


Gtk in python with gobject
========================================
install
----------------------------------------
$sudo aptitude install gobject-introspection

usage
----------------------------------------
>>> from gi.repository import Gtk



mercurial hg ssh
========================================

hg clone <org.file> ssh://pi@raspi.local/<dirName>

at pi@raspi, ~/<dirName>/<files>

.. EOF

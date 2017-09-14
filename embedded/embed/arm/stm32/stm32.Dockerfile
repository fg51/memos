# master iamges
FROM ubuntu:14.04

RUN apt-get update
#RUN apt-get install -y build-essential

# install add-apt-repository
#
# $ apt-file search add-apt-repository
# software-properties-common: /usr/bin/add-apt-repository
# software-properties-common: /usr/share/man/man1/add-apt-repository.1.gz
RUN apt-get install -y apt-file
RUN apt-file update
RUN apt-get install -y software-properties-common


# install the launchpad's arm-eabi
# CAUTION: check the new version.
#
RUN apt-get remove -y binutils-arm-none-eabi gcc-arm-none-eabi
RUN add-apt-repository -y ppa:terry.guo/gcc-arm-embedded
RUN apt-get update && apt-get install -y gcc-arm-none-eabi=4.9.3.2015q3-1trusty1

# install the stlink
# installed command: st-flash, st-info, st-term, st-util
#
RUN apt-get install -y dh-autoreconf pkg-config libusb-1.0-0-dev git sudo
RUN git clone https://github.com/texane/stlink /root/stlink
WORKDIR /root/stlink
RUN ./autogen.sh && ./configure && make && make install
RUN cp -f 49-stlinkv*.rules /etc/udev/rules.d
RUN udevadm control --reload-rules

RUN useradd -G sudo -m -s /bin/bash docker \
  && echo 'docker:docker' | chpasswd
USER docker
WORKDIR /home/docker
ENV HOME /home/docker

#RUN apt-get install -y sudo
#RUN useradd -G sudo -m -s /bin/bash <username> && \
#    echo '<username:pass>' | chpasswd
#

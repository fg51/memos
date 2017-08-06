Docker
========================================

[Dockerfile ベストプラクティス (仮)](https://www.qoosky.net/techs/207)


## require

need the kernel more than 3.10.

```sh
$ uname -r
3.11.0-15-generic
```

```sh
$ uname -r
4.3.3-2-ARCH
```


## install

```sh
$ apt-key adv \
    --keyserver hkp://pgp.mit.edu:80 \
    --recv-keys 58118E89F3A912897C070ADBF76221572C52609D
```

```sh
$ cat << EOF > /etc/apt/sources.list.d/docker.list
# Ubuntu Trusty
deb https://apt.dockerproject.org/repo ubuntu-trusty main
# Ubuntu Vivid
deb https://apt.dockerproject.org/repo ubuntu-vivid main
# Ubuntu Wily
deb https://apt.dockerproject.org/repo ubuntu-wily main
```

remove old lxc

```sh
$ sudo apt-get purge lxc-docker*
```

verify that apt is pulling from the right repository.

```sh
$ apt-cache policy docker-engine
```


### verify docker

```sh
$ sudo docker info
$ sudo docker run hello-world
```


### without sudo

because docker-command use socket (/var/run/docker.sock) to access the Docker server,
user need sudo.


```sh
re-login
$ sudo gpasswd -a <user> docker
adding <user> to group docker
```


## download image

$ docker pull <image>:<tag>

$ docker pull ubuntu:latest
$ docker pull ubuntu:14.04

NOTE: default access the "Docker Hub Registry"


## check local image

```sh
$ docker images
```


## create & execute container

```sh
$ docker run [option] [--name {container name}] {image} {: {tag name}][command][arg]
```

options:
-d: background
-i: open stdio
-t: keep tty
-p: {port of host}:{port of container}



https://hub.docker.com


$ docker ps -a



## with GUI

```sh
$ xhost local:host
$ docker run -it -e DISPLAY=$DISPLAY -v=/tmp/.X11-unix:/tmp/.X11-unix -v ~/.Xauthority:/root/.Xauthority <image:tag> /bin/bash
env $ firefox
```

```sh
$ xhost +local:root

docker run -it -e DISPLAY=:0.0 \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v ~/.Xauthority:/root/.Xauthority \
    -v /dev/bus/usb:/dev/bus/usb \
    -v <user/directory>:<env/directory>
    <image:tag> <app>
```


## docker commit

1. exit container:
ctrl + p or ctrl + q 


2. new container

```sh
$ doccker commit default <dest container>
```

2.  same container with tag

```sh
$ doccker commit <org contaireID> <dest container>:<tag name>
```

3.  check

```sh
$ docker images
```

```sh
$ docker tag <ID> <dest>:<tag>
```


## remove container

```sh
$ docker ps -aq |xargs docker rm
```

```sh
bash$ docker rm `docker ps -a -q`
```


## remove image

```sh
$ docker rmi <image:tag>
```


## run with user
```sh
$ docker run -i -t <image_name> su - <username>
```


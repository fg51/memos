dockerfile
====

[効率的に安全な Dockerfile を作るには - Qiita](http://qiita.com/pottava/items/452bf80e334bc1fee69a)


## build command

```bash
$ docker build -t <image name> <dockerfile directory>
```

## dockerfile commands

```bash
$ cat Dockerfile
FROM <target image>

```

RUN
execute at "$ docker build"

CMD
execute at "$ docker run"

ENTRYPOINT
execute at "$ docker run"

## sample

```sh
$ cat dockerfile
FROM ubuntu
MAINTAINER username <xxxx@gmail.com>
RUN apt-get install -y nginx
ADD index.html /usr/share/nginx/html/

$ echo 'Hello!' > index.html
$ ls
Dockerfile  index.html

$ docker build -t sample

```


## practice

1. docker run -it <image> bash
2. use cp -f ln -f, curl -s wget -q
3. if failed then exit and go to #1. else go to 4.
4. save exit & docker commit



```sh
$ docker run --rm -it --privileged -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix -v ~/.Xauthority:/root/.Xauthority -v /dev/bus/usb:/dev/bus/usb -v ~/work/mplab_project/projects:/root/work jiita/lpcxpr:v326  bash
```
TODO: need user
... <image> su - <user> ?


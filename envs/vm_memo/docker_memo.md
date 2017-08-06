DOCKER
========================================

[bootdocker](http://qiita.com/k2works/items/926c4a3f5798c47b4088)


## install


## upgrade

```sh
$ boot2docker stop
$ boot2docker download
$ boot2docker start
```

## execute

```sh
docker run ubuntu echo hello world
hello world
```


# in Ubuntu 14.04

```sh
$ vagrant init phusion/ubuntu-14.04-amd64

...
$ script = <<SCRIPT
    echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf > /dev/null
  SCRIPT

  config.vm.provision "shell", inline: $script
...
```

## setup
$ vagrant ssh
$ sudo apt-get update
$ sudo apt-get install docker.io
$ sudo ln -sf /usr/bin/docker.io /usr/local/bin/docker
$ sudo sed -i '$acomplete -F _docker docker' /etc/bash_completion.d/docker.io


$ sudo docker run -i -t ubuntu /bin/bash
Unable to find image 'ubuntu' locally
Pulling repository ubuntu
e54ca5efa2e9: Download complete
3db9c44f4520: Download complete
195eb90b5349: Download complete
c5881f11ded9: Download complete
463ff6be4238: Download complete
ebe4be4dd427: Download complete
511136ea3c5a: Download complete
6cfa4d1f33fb: Download complete
d7ac5e4f1812: Download complete
4d289a435341: Download complete
bac448df371d: Download complete
3af9d794ad07: Download complete
f127542f0b61: Download complete
dfaad36d8984: Download complete
fae16849ebe2: Download complete
994db1cb2425: Download complete
b7c6da90134e: Download complete
5796a7edb16b: Download complete
0f4aac48388f: Download complete
f86a812b1308: Download complete
47dd6d11a49f: Download complete
209ea56fda6d: Download complete
0b628db0b664: Download complete
2f4b4d6a4a06: Download complete
83ff768040a0: Download complete
6c37f792ddac: Download complete
root@6ab798be1467:/#

ubuntuイメージをダウンロードしてコンテナ内のbashが実行されれば成功。


## 追加設定

$ exit
$ vagrant plugin install sahara
$ vagrant sandbox on
$ vagrant ssh




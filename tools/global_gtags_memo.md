gnu global
====
[GNU GLOBALの対応言語を大幅に増やすPygmentsパーサーを導入する - Qiita](https://qiita.com/yoshizow/items/9cc0236ac0249e0638ff)

applied pygments : > global-6.3.2


## required

### for build

```sh
$ sudo apt install automake autoconf gperf bison flex texinfo
```

### pygments

```sh
$ pip install pygments
```


## intall

```sh
$ VERSION=6.6.1
$ wget http://tamacom.com/global/global-${VERSION}.tar.gz
$ tar xvf global-${VERSION}.tar.gz
$ cd global-${VERSION}
$ ./configure --prefix=$HOME/.local/share/global
$ make
$ make install
```


## config

```sh
$ cp /usr/local/share/gtags/gtags.conf ~/.globalrc
```

```.globalrc
default:\
-    :tc=native
+    :tc=native:tc=pygments:
```

```sh
$ cp /usr/local/share/gtags/gtags.vim $HOME/.vim/plugin
$ cp /usr/local/share/gtags/gtags-cscope.vim $HOME/.vim/plugin
```

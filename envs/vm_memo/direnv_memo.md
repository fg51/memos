direnv
========================================

[direnv](http://d.zinrai.info/blog/html/2014/06/07/debian_wheezy_direnv_xbuild.html)

require golang


## install

```bash
$ git clone https::/github.com/zimbatm/direnv.git
$ cd direnv
$ make
$ ln -s direnv <bin dir>
($ make install DESTDIR=<install target dir>)


$ HOME/.zshrc
type direnv > /dev/null
#if [$? -eq 0]; then
if [$? != 0]; then
    eval "$(direnv hook zsh)"
fi
```


## usage

```bash:.envrc
$ cd <dest dir>
$ direnv edit .
export PATH=bin:$PATH
$ direnv allow
```




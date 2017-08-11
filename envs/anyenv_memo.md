Anyenv
========================================

## require:

[Common-build-problems](https://github.com/yyuu/pyenv/wiki/Common-build-problems)

```bash
sudo apt-get install -y make build-essential libssl-dev zlib1g-dev libbz2-dev \
libreadline-dev libsqlite3-dev wget curl libncurses5-dev
llvm
```


## install

[anyenv](https://github.com/riywo/anyenv)

```bash
$ git clone http://github.com/riywo/anyenv ~/.anyenv
$ echo 'export PATH="$HOME/.anyenv/bin:$PATH"' >> ~/.zshrc
$ echo 'eval "$(anyenv init -)"' >> ~/.zshrc
$ exec $SHELL -l

$ pyenv / anyenv : need "rehash"

$ cat config.log | grep checking
```



## Pyenv with anyenv

### ref
[pyenv-with-virtualenv](Http://blog.ieknir.com/blog/pyenv-with-vritualenv/)

### Install

```bash
$ anyenv install pyenv
$ vim ~/.zshenv
    export PYENV_ROOT="${HOME}/.anyenv/envs/pyenv"
    if [ -n ${PYENV_ROOT} ]; then
        path=(${PYENV_ROOT}/bin ${PYENV_ROOT}/shims ${path})
    fi
    eval "$(pyenv virtualenv-init -)"
$ source ~/.zshenv
$ git clone git://github.com/yyuu/pyenv-virtualenv.git ~/.anyenv/envs/pyenv/plugins/pyenv-virtualenv
```


```fish
$ git clone http://github.com/riywo/anyenv ~/.anyenv
$ cat << EOF >> $HOME/.confi/fish/config.fish
set -gx PATH $PYENV_ROOT/shims $PYENV_ROOT/bin $PATH"
pyenv rehash > /dev/null ^&1
EOF
```

$ git clone http://github.com/yyuu/pyenv-virtualenv $PYENV_ROOT/plugins/pyenv-virtualenv


### usage

```bash
$ pyenv install -l
$ pyenv install <version>
$ pyenv rehash

$ pyenv versions
$ pyenv virtualenvs
# $ pyenv mkvirutalenv <version> <env name>
$ pyenv virutalenv <version> <env name>

$ pyenv global 2.7.10 3.4.2  #applied: python 3.4.2
$ pyenv shell 2.7.10 3.4.2  #applied: python 3.4.2
or
$ pyenv shell <env3> <env3>
(env name) $ 
$ pyenv shell

```


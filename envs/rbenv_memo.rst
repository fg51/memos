rbenv
========================================

install
----------------------------------------
anyenv install the rbenv with ruby-build

.. code:: ruby
    $ git clone https://github.com/sstephenson/ruby-build.git ~/.rbenv/plugins/ruby-build


system install
    $ git clone https://github.com/sstephenson/ruby-build.git /usr/local/anyenv/envs/rbenv/plugins/ruby-build


echo 'export PATH="HOME/.anyenv/bin:$PATH" >> <your_profile>
eval "$(anyenv init -)" >> <your_profile>


echo 'export PATH="HOME/.anyenv/env/rbenv:$PATH" >> <your_profile>
eval "$(anyenv init -)" >> <your_profile>


anyenv-install not definition 
----------------------------------------
ANYENV_ROOT is not defined
echo 'ANYENV_ROOT="/usr/local/anyenv"' >> <your_profile>


install anyenv in the system
========================================
.. code:: bash
    $ sudo cat > /etc/profile.d/anyenv.sh
    export ANYENV_ROOT="/usr/local/anyenv"
    export PATH="$ANYENV_ROOT/bin:$PATH"
    eval "$(anyenv init -)"

    export RBENV_ROOT="$ANYENV_ROOT/envs/rbenv"
    export PATH="RBENV_ROOT/bin:$PATH"
    eval "$(rbenv init -)"

    export PYENV_ROOT="$ANYENV_ROOT/envs/pyenv"
    export PATH="PYENV_ROOT/bin:$PATH"
    eval "$(pyenv init -)"

applied ruby
========================================

$ rbenv install 2.2.2

RBENV_ROOT=$ANYENV_ROOT/envs/rbenv
$ echo 'export PATH="HOME/.anyenv/env/rbenv$PATH" >> <your_profile>
$ eval "$(anyenv init -)" >> <your_profile>



install bundler
========================================
$ gem install bundler --no-rdoc --no-ri




$ sudo yum install gcc make openssl openssl-devel

ruby with rbenv
----------------------------------------
$ cd /usr/local/
$ sudo git clone git://github.com/sstephenson/rbenv.git rbenv
$ mkdir -p rbenv/shims rbenv/versions
$ sudo groupadd /usr/local/rbenv
$ sudo chgrp -R rbenv rbenv/

$ sudo git clone git://github.com/sstephenson/ruby-build.git /usr/local/rbenv/ruby-build
$ cd /usr/local/rbenv/ruby-build
$ ./install.sh

環境変数設定ファイルの作成
----------------------------------------
$ cat >> /etc/profile.d/rbenv.sh << _EOF_
export ANYENV_ROOT="/usr/local/anyenv"
export PATH="$ANYENV_ROOT/bin:$PATH"
eval "$(anyenv init -)"

export RBENV_ROOT="$ANYENV_ROOT/envs/rbenv"
export PATH="RBENV_ROOT/bin:$PATH"
eval "$(rbenv init -)"
_EOF_

$ source /etc/profile.d/rbenv.sh

$ sudo rbenv install -v 2.1.5

check
----------------------------------------
$ rbenv versions
* system (set by /usr/local/rbenv/version)
  2.1.3
$ sudo rbenv global 2.1.3
$ rbenv versions
  system
* 2.1.5 (set by /usr/local/rbenv/version)
$ ruby --version
ruby 2.1.5p273 (2014-11-13 revision 48405) [x86_64-linux]
$


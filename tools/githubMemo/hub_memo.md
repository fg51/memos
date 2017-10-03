hub command
====

## ref

[GitHub のコマンドラインツール「hub」の基本と便利な使い方のまとめ ｜ Developers.IO](http://dev.classmethod.jp/tool/git/hub/)


## install

```sh
$ pacman -S hub
```

```sh
$ go get github.com/github/hub
```


### completion

#### fish

```sh
cp hub/etc/hub.fish_completion ~/.config/fish/completion/hub.fish
```

#### bash

.bashrc

```sh@.bashrc
if [ -f /path/to/hub.bash_completion ]; then
  . /path/to/hub.bash_completion
fi
```

#### zsh

Copy the file etc/hub.zsh_completion from the location where you downloaded hub to the folder ~/.zsh/completions/ and rename it to _hub:

```sh
$ mkdir -p ~/.zsh/completions
$ cp hub/etc/hub.zsh_completion ~/.zsh/completions/_hub
```

```sh@.zshrc
fpath=(~/.zsh/completions $fpath) 
autoload -U compinit && compinit
```


## usage

### git create

```sh
$ git create [NAME] [-p] [-d DESCRIPTION] [-h HOMEPAGE]
```

NAME: repository name
-p : private repository
-d DESCRIPTION : detail of repository
-h HOMEPAGE : add homepage


### git browse

git browse [-u] [[USER/]REPOSITORY] [SUBPAGE]

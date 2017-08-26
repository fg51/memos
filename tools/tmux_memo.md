tmux
====


## install

```sh
$ pacman -Syu tmux
$ tmux -V
tmux 2.2
```

caution: tmux read byobu-config ? at running byobu



## plugin manager: TPM

### install

```sh
$ git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
```


```sh@~/.tmux.conf
$ cat << EOF >> ~/.tmux.conf
set -g @plugin 'tmux-plugins/tpm'


# END LINE
#Initialize TMUX plugin manager (keep this line at the very bottom of tmux.conf)
run '~/.tmux/plugins/tpm/tpm'
```

### usage

```diff@~/.tmux.conf
$ diff ~/.tmux.conf
set -g @plugin 'tmux-plugins/tpm'

+ set -g @plugin `tmux-plugins/foo`
+ set -g @plugin `tmux-plugins/foo`

#Initialize TMUX plugin manager (keep this line at the very bottom of tmux.conf)
run '~/.tmux/plugins/tpm/tpm'
```

install plugins in tmux

* push prefix + I : install
* push prefix + U : update
* push prefix + alt + u : unsintall


### plugins

tmux-sensible : tmuxの基本設定
tmux-pain-control : ペイン操作のキーバインド追加
tmux-logging : ロギングとスクリーンキャプチャ
tmux-yank : システムのクリップボードにコピー
tmux-resurrect : tmux環境の保存と復元
tmux-continuum : tmux-resurrectと組み合わせることで自動で実行
tmux-copycat : 表示内容を正規表現で検索
tmux-open : ハイライトしているファイルやURLを開く
tmux-battery : バッテリーの状態をステータスバーに表示


### tmux-logging

* prefix + shift + p : start logging
* prefix + alt + p : screen capture
* prefix + shift + alt + p : stop logging



### tmux-pain-control

#### Navigation

* prefix + h and prefix + C-h : select pane on the left
* prefix + j and prefix + C-j : select pane below the current one
* prefix + k and prefix + C-k : select pane above
* prefix + l and prefix + C-l : select pane on the right

Note: This overrides tmux's default binding for toggling between last active windows, prefix + l. tmux-sensible gives you a better binding for that, prefix + a (if your prefix is C-a).


#### pane resizing

Resizing panes

* prefix + shift + h : resize current pane 5 cells to the left
* prefix + shift + j : resize 5 cells in the up direction
* prefix + shift + k : resize 5 cells in the down direction
* prefix + shift + l : resize 5 cells to the right

These mappings are repeatable.

The amount of cells to resize can be configured with @pane_resize option. See configuration section for the details.


#### pane splitting

Splitting panes

* prefix + | : split current pane horizontally
* prefix + - : split current pane vertically


### tmux-yank

* prefix + y : copy

( default copymode: prefix + [ )


### tmux-copy

1. prefix + [ : start select mode with vi
3. space: start copy
4. Enter: finish copy
5. Prefix + ] : paste

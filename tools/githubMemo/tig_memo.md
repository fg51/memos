tig
====

* [超絶便利なGitクライアントのtigのすすめ - Qiita](https://qiita.com/vivid_muimui/items/7e7a740e6537749de0c0)
* [Tig で Git を自由自在に操作するための .tigrc 設定例 - Qiita](https://qiita.com/sfus/items/063797a1dd8fdc7d032f)

q: close the view
Q: quit tig

j, k: up, down
<-, ->: scroll left, right

## view list

m	main	git tree
s	status	git status
r	refs	branch, tag list
l	log		git log
t	tree	directory tree
g	grep	git grep
b	blame	git blame
h	help	help


## commit log list

```sh
$ tig
```

### select commit

* up: <up>, <ctrl + p>
* down: <down>, <ctrl + n>


### diff

* page up: <pageUp>, ctrl + u
* page down: <pageDown>, ctrl + d


## status

```sh
$ tig status
```

### select commit

* up: <up>, <ctrl + p>
* down: <down>, <ctrl + n>


### diff

* page up: <pageUp>, ctrl + u
* page down: <pageDown>, ctrl + d


### add, reset, commit

u: git add, or git reset HEAD
!: git checkout
C: commit


## file tree

```sh
$ tig tree
```



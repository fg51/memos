git
====


## clone any branch

```sh
$ git branch -r
origin/HEAD -> origin/master
origin/develop
origin/master

$ git checkout -b develop origin/develop
```


## cancel rebase

```sh
$ git reset --hard ORIG_HEAD
```

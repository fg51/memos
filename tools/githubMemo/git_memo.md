# git

## commit

dev - feat (commit1, commit2, commit3 ...) > dev - feat (commit)

```sh
git rebase -i develop
```

(develop)  git merge (--ff-only) feature
(main)  git merge (--no-ff) develop

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
git reset --hard ORIG_HEAD
```

## file permision

```sh
git config core.filemode false
```

## git worktree

use wt (worktrunk)

### create with exsisted branch

```sh
$ pwd
/path/to/src.git

$ git worktree add ../path/to/dest your/branch
```

### create with new branch

```sh
git worktree add -b feat1 ../path/to/dir
```

### delete

```sh
git worktree remove ../path/to/dir
```

or

```sh
rm ../path/to/dir
git worktree prune
```

## rebase

```sh
git rebase -i base-branch
```

## merge

```sh
(at dev) git merge --ff-only feat/xxx
```

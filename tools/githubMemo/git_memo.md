# git

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

### create

```sh
git worktree add ../path/to/dir your/branch
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

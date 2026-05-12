# git

## fetch local repository

```sh
# in local repository
git remote add workb /path/to/repob
git fetch workb
git merge workb/feature
```

### for remote bare repository

```sh
git remote -v
origin /path/to/repo.git
workb /path/to/repob

git push origin main

git push -u origin main  # set upstream
git push  # to upstream = origin main


```

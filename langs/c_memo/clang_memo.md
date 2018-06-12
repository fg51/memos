clang
====


## memory analysis

```sh
$ clang -fsanitize=addres -fsanitize=leak
```


## clang-format

###  usage

```sh
$ clang-format -i <src>
```

### config

```sh
$ clang-format -style=llvm -dump-config > .clang-format
$ cat .clang-format
---
BasedOnStyle: LLVM
...

$ cp .clang-format $HOME
$ clang-format -i -style=file <src>
```

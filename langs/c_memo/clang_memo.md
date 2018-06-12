clang
====


## memory analysis

```sh
$ clang -fsanitize=addres -fsanitize=leak
```


## clang-format

```sh
$ clang-format -i <src>
```

```sh
$ cat .clang-format
---
BasedOnStyle: LLVM
ColumnLimit: 110
BinPackParameters: false

$ clang-format -i -style=file <src>
```

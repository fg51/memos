msgpack
====


## 1. install

```sh
$ git clone https://github.com/msgpack/msgpack-haskell .

$ diff stack.yaml
  packages:
  - '.'
+ - msgpack/

$ diff my-project.yaml
  library
    build-depends:       base >= 4.7 && < 5
+                      , msgpack

```


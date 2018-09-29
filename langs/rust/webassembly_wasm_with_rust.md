webassembly with rust
====

## setup

```sh
$ rustup show
$ rustup target list | ag thumbv
$ rustup target add wasm32-unknown-unkonwn
```

```sh
$ cargo install cargo-generate
$ cargo install wasm-pack
```

## create project
```sh
$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
$ wasm-pack build  # NOTE: install wasm-bindgen
# DEPRECATED: $ wasm-pack init  # NOTE: install wasm-bindgen
#$ cargo generate --git https://github.com/rustwasm/wasm-webpack-template
```


```sh
$ npm init wasm-app www
```

```sh
$ cd $PROJECT/www
$ npm install

$ cd /path/to/$PROJECT/pkg
$ npm link

$ cd /path/to/$PROJECT
$ npm link $PROJECT
```

## build

```javascript@$PROJECT/www/index.js
- import * as wasm from "hello-wasm-pack";
+ import * as wasm from "$PROJECT;

wasm.greet();
```

```sh
$ cd $PROJECT/www
$ rpm start
```

## Public methods, exported to JavaScript.

```rust
#[wasm_bindge]
impl Foo {
  pub fn bar() {

}
```

webassembly with rust
====

## toolchain

* wasm-bindgen: basic types.
* js-sys: use JavaScript's value via rust.
* web-sys: use DOM via rust.

* cargo-generate: get template
* wasm-pack: wasm-pack helps you build and publish rust-generated WebAssembly to the npm registry.
* (Webpack: build wasm-binary to npm)
* (wasm-pack use wasm-bindgen-cli)


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
$ cd /path/to/$PROJECT
$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
$ tree
.
+ Cargo.toml
+ LICENSE_APACHE
+ LICENSE_MIT
+ README.md
+ src
  + lib.rs
  + utils.rs
$ wasm-pack build  # NOTE: install wasm-bindgen
# DEPRECATED: $ wasm-pack init  # NOTE: install wasm-bindgen
#$ cargo generate --git https://github.com/rustwasm/wasm-webpack-template
```


```sh
$ npm init wasm-app www
```

```sh
$ cd /path/to/$PROJECT/www
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

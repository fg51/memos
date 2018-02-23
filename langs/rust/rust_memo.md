Rust
====

## prepare

```sh
$ set -gx PATH $HOME/.cargo/bin $PATH
```

```sh
$ export PATH="$HOME/.cargo/bin:$PATH"
```


## install

```sh
$ curl https://sh.rustup.rs -sSf | sh
```

```sh
$ rustc --version
$ rustup update
$ rustc --version

$ rustup install nightly
$ rustup run nightly rustc --version
$ rustup default nightly
```

### example

```rust
fn main() {
  println!("Hello, World!");
}
```

```sh
$ rustc hello.rs && ./hello
Hello, World!
```


## create project

cargo is the package-manager.

```sh
$ cargo new hello --bin
$ tree hello
hello
├── Cargo.toml
├── .git
├── .gitignore
└── src
    └── main.rs
$ cargo build --release
```


## formatter

cargo install rustfmt

```sh
$ rustup component add rust-src
```

```sh
$ cd project
$ cargo fmt
```

### config:

rustfmt.toml


### vim

[GitHub - rust-lang/rust.vim: Vim configuration for Rust.](https://github.com/rust-lang/rust.vim)


## complete

### Racer

```sh
$ cargo install racer
```




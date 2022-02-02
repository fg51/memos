```sh
$ sudo apt-get install libudev libusb-1.0.0-dev
```

```sh
$ cargo install cargo-binutils
$ rustup +stable component add llvm-tools-preview
$ cargo add cargo-embed
$ cargo install flip-link
$ cargo install probe-run
```


```sh
$ cargo generate \
  -- git https://github.com/rtic-rs/app-template \
  --branch main \
  --name my-app
```

```sh
$ probe-run --list-chips
```


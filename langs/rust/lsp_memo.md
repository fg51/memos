```sh
$ rustup update
$ rustup component add rls-preview --toolchain nightly
$ rustup component add rust-analysis --toolchain nightly
$ rustup component add rust-src --toolchain nightly
```

[[plugins]]
repo = 'rust-lang/rust.vim'
on_ft = 'rust'
hook_source = '''
 let g:rustfmt_autosave = 1
'''

Golang
====


## setup go-tool

```sh
go get golang.org/x/tools/cmd/vet
go get golang.org/x/tools/cmd/goimports
go get golang.org/golang/lint/golint
```

## for vim

### plugin
vim-jp/vim-go-extra

### vimrc
set runtimepath+=$GOPATH/src/github.com/golang/lint/misc/vim
set runtimepath+=$GOPATH/src/github.com/nsf/gocode/nvim
autocmd FileType go autocmd BufWritePre <buffer> Fmt

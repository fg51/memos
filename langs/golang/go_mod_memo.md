go modules
====

old name: vgo


## init
```sh
$ cd /path/to/app
$ go mod init github.com/you/hello

$ cat go.mod
module github.com/you/hello

go 1.12
```

## sample

```go
package main

import (
	"fmt"
	"rsc.io/quote"
)

func main() {
	fmt.Println(quote.Hello())
}
```

### get module1
```sh
$ go build  # or go get rsc.io/quote
$ ls
go.md
go.sum
hello
hello.go
$ ls ~/.go/pkg/mod/
rsc.io
```

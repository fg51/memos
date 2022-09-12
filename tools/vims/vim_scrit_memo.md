# 1 output in command

```vim
let mess = "Hello Vim Script World"
echo mess
```

run with command-mode in vim
:source hello.vim

output in command : 'Hello Vim Script World' 


# 2 output in buffer

```vim
let mess = "Hello Vim Script World"
call setline('.',  mess)
```

run with command-mode in vim
:source hello.vim

output in buffer : 'Hello Vim Script World'

# 3 to command

## 3.1 plugin path
```
:source &packpath
```

## 3.2 autoload
```
function! hello#execute(start, end) abort
  let mess = 'Hello Vim Script World'
  call setline('.', mess)
endfunction
```


## 3.3 plugin
```
let s:save_cpo = &cpo
set cpo&vim

scriptencoding utf-8

if exist('g:loaded_hello')
  finish
endif

let g:loaded_hello = 1

command! -range Hello call hello#execute(<line1>, <line2>)

let &cpo = s:save_cpo
unlet s:save_cpo
```

## 3.4 run
run with command-mode in vim
:Hello

output in buffer : 'Hello Vim Script World'


# 
" is comment-out

## 
* number is 1,  1.5,  0x1E
* string is 'a',  "a"
* list is [1, 2, 3]
* dict is {'name': 'godzilla'}


## add string
" hello gorilla
echo 'hello' . 'gorilla'

## variable
let name = 'foo'
let name = 'bar'  " assignment

## scope
g: is the global scope
s: is the script scope
l: is the local scope in function
a: is the local scope in function args
v: is the global scope, embedded variable

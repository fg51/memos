
[Qiita](https://qiita.com/kaityo256/items/cb76c3f73753fe921e7b)


```vimrc
augroup vimrc-local
  autocmd!
  autocmd BufNewFile,BufReadPost * call s:vimrc_local(expand('<afile>:p:h'))
augroup END

function! s:vimrc_local(loc)
  let files = findfile('.vimrc.local', escape(a:loc, ' ') . ';', -1)
  for i in reverse(filter(files, 'filereadable(v:val)'))
    source `=i`
  endfor
endfunction
```


```.vimrc.local
let g:ale_cpp_clang_options = "-std=c++14 -Wall -I../inc"
let g:ale_cpp_gcc_options = "-std=c++14 -Wall -I../inc"
```

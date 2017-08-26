X clipboard
====


## buffers

* PRIMARY
* SECONDARY
* CLIPBOARD


## tools

* xsel: control x-selection(clipboard)
* xclip: cli for x-selection(clipboard)


## xclip

* -i : 標準入力またはファイルから、バッファにコピーする
* -o : 標準出力にバッファの内容をコピーする
* -selection : buffer (PRIMARY,SECONDARY,CLIPBORD (default: PRIMARY ))


```bash
$ echo "test" | xclip -i
$ xclip -o
test
```

## xsel

stdin only

```bash
% cat test.txt | xsel     # PRIMARY
% cat test.txt | xsel -b  # CLIPBOARD
```

```bash
% xsel
text.txtの内容
% xsel -b
text.txtの内容
```

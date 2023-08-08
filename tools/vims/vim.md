vim
====

## operator, motion, text-object, repeat

* operator is update, delete, yank, select. ex. d
* motion is move cursor. ex. d $
* textobject is text. ex. d iw
* repeat is '.'

see help index

{count}{operator}{textobjecct}


## 1. basic
### cursor
* left: h
* down: j
* up: k
* right: l
* top: gg
* bottom: G
* move with count: {count}G
* move to top in display: z<CR>
* move to middle in display: zz
* move to end in display: zb


### insert
I: move top of line and into insert-mode
a: move right and into insert-mode
A: move end of line and into insert-mode
o: add next line and move to next line and into insert-mode
O: add before line and move to before line and into insert-mode


### visual
v: char unit
V: line unit
Ctrl-v:

## 2. normal mode
### count

### motion
0: move to top of line. (include blank)
^: move to top of line. (exclude blank)
$: move to end of line.
g_: move to end of line. (exclude blank)
{count}f{char}: move to right {count}th of {char}.
{count}F{char}: move to left {count}th of {char}.
{count}t{char}: move to right {count}th of {char} and move left.
{count}T{char}: move to left {count}th of {char} and move right.
{count}; : repeat {count} last f/F/t/T.
{count}, : repeat {count} last f/F/t/T.
G:
gg:
{count}w: next {count} top of word.
{count}W: next {count} top of word(without blank).
{count}b: before {count} top of word.
{count}B: before {count} top of word(without blank).
{count}e: next {count} end of word.
{count}E: next {count} end of word(without blank).
{count}}: next {count} blank line
{count}{: before {count} blank line
%: move to (/[/{/}/]/)

### operator
y: yank
p: put (paste).
d: delete
c: delete and into insert-mode.
gU: to upper char. ex. gUw
gu: to small char. ex. guw
{num of line}>: add indent with {num of line}.
{num of line}<: delete 1 indent.
{num of line}zf: folding {num of line}.
za: open folding
zn: open all of folding

### text object
iw: select word. ex. gUi"
aw: elsect word. ex. da"
i]
a]
i)
a)
i>
a>
i": select "{***}". ex. gUi"
a": elsect {"***}". ex. da"
i'
a'

q
:q
q!
:q

### others
u: undo
~: change upper/small char.
J: connect line with blank.
gJ: connect line without blank.


## command mode
:new
:vnew
:sp
:vs


### external command
:!{cmd}: run {cmd}
:r : add stdout into current file.

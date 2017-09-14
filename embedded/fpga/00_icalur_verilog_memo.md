ICARUS VERILOG
========================================

[Icarus Verilog](http://iverilog.icarus.com/)


## install

```bash
$ sudo apt-get install iverilog gtkwave
```


## usage

```bash
# compile
$ iverilog -o <out.vvp> <src>

# do simulate (output: ***.vcd as wave file)
$ vvp <out.vvp>
```

compile options

-o <out file name>
-s <top module name>


## view wave result

```bash
$ gtkwave <out.vcd> (***.sav)
```

".sav" file is the format file (saved last view form)
[File] -> [Write Save File As] or [File] -> [Write Save File]

"append" or "insert" button add the selected signals



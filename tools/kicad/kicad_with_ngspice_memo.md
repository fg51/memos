
## analysis commands

for DC analysis
DC 0

for AC analysis
AC 0

for transient analysis
sin(0.0 1.0 100k) : DC offset: 0V, Amplitude: 1V, frequency: 100kHz


## add spice script

Place > Graphic text (key: t)

```spice
+PSPICE
.include <file.name>
```

```spice
+PSPICE
.include ./run.com
```

## output

check "Default format"
check "prefix references 'U' and 'IC' with 'X'"
push "Generate"


## spice script sample

```@run.com
* setup
.options temp=20 tnom=20

* step: 0.1us, end: 100us
.Tran  0.1us 100us


.control
run

wrdata result.txt V(Vinn) V(Vout)
```

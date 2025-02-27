# ltspice

## component

### behavior voltage

keyword: "bv"

```spice
V = {formula} or {label}
V =  0.5 * sin(2 * pi * 1k * time) + V(IN)
V = white(2 * pi * 100k * time)  # white noise
```

#### with laplace, behavior voltage

```spice
laplace = 1 / (1 + 100e-6 * 6 * s)  # 1st low pass filter
```

apply: ideal op-amp

### load, current source

keyword: "load2"

set current value

### voltage source cotroled voltage

```spice
10  # Vout = 10 * Vin
```

keyword: "e"

## to component

Edit > hierarchy > open this sheet's symbol > couldn't find ... [YES]

## simulation

### DC sweep

```spice
.dc {label1} {start1} {end1} {step1} {label2} {start2} {end2} {step2}
.dc V1 0 15 10m I1 20u 100u 20u
```

## batch

caution: without "Automatically delte .raw files"

```sh
$ {ltspice}.exe -netlist {/path/to/target.asc}  # generate {target.net}
$ {ltspice}.exe -b {/path/to/target.net}
# see {out.raw}
```

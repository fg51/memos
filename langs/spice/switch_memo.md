Switch Model (SW/CSW)
====

## parameter

- SW: voltage switch
- CSW: current switch


```eval_rst
===== =================== ====== ======== =============
Name  parameter           Units  Default  Switch model
===== =================== ====== ======== =============
VT    threshold-voltage   V      0.0      SW
IT    threshold-current   A      0.0      CSW
VH    hysteresis-voltage  V      0.0      SW
IH    hysteresis-current  A      0.0      CSW
RON   on resistance       ohm    1.0      SW,CSW
ROFF  off resistance      ohm    1.0E+12  SW,CSW
===== =================== ====== ======== =============
```

## node

SXX N+ N- NC+ NC- model <on><off>

example:
S1 Vout 0 sig 0 S1 ON

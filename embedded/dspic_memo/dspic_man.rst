dsPIC33FJ16MC102




.. sourcecode:: cpp
    #ifdef __cplusplus
    extern "C" {
    #endif 
    
    #include "xc.h"
    #include "stdbool.h"

    #ifdef __cplusplus
    }
    #endif 
 
ref
========================================
MPLABX-IDE XC32 コンパイラでアセンブルリストを出力
http://d.hatena.ne.jp/inouema/touch/20130109/1357717885


output assemble
========================================

.. sourcecode:: bash

    $ xc32-gcc.exe -mprocessor=<mpu name> -S <dest.asm> <org.c>

with mplabx
----------------------------------------
check the box: "Execute this line before build"
    write "xc32-gcc.exe" -mprocessor=<mpu name> -S <dest.asm> <org.c>

CLOCK
========================================
Fsys = 1 / 4 * input_freq * multiplication of PLL 

_FOSC(<TYPE& <MODE> & <SWITCH>

type
----------------------------------------
:    : external clock (oscillator)
:LP  :  32 [kHz] internal clock (using the clock for Timer1)
:FRC :   8 [MHz] internal clock
:LPRC: 512 [kHz] internal clock

MODE
----------------------------------------

with external oscillator
:XT         : * 1
:XT_PLL4    : * 4
:XT_PLL8    : * 8
:XT_PLL16   : * 16
:XTL        : * 1 (set at <  4 [MHz])
:HS         : * 1 (set at < 10 [MHz])

internal clock without external oscillator
:FRC_PLL4   : * 4
:FRC_PLL8   : * 8
:FRC_PLL16  : * 16

high freq (> 4 [MHz]) with external clock
:EC         : pin OSC2 output the 1 / 4 freq for the inputed clock
:ECIO       : pin OSC2 used as I/O 
:EC_PLL4    : * 4  with the external clock
:EC_PLL8    : * 8  with the external clock
:EC_PLL16   : * 16 with the external clock

low freq (< 4 [MHz]) with external clock
:ERC        : pin OSC2 output the 1 / 4 freq for the inputed clock
:ERCIO      : pin OSC2 used as I/O 


SWITCH
----------------------------------------
:CSW  : Clock Switch
:FSCM : Fail Safe Clock Monitor

:CSW_FSCM_OFF:
:CSW_ON_FSCM_OFF:
:CSW_FSCM_ON: 


MCPWM
========================================

ref
----------------------------------------
http://www.maroon.dti.ne.jp/koten-kairo/works/dsPIC/motor5.html

.. sourcecode:: cpp
    unsigned int period ///< max count for PWM freq


PWM freq = System clock / ((period + 1) * prescaler)
for Example:
    Fsys = 20 [MHz]
    period = 311
    prescaler = 64 (1:64)
    20E+6 / ((311 + 1) * 64) = 1001.6 = 1 [kHz]


========================================
mplabx
========================================

Config Bits の設定
----------------------------------------
#. Window > PIC Memory Views > Configuration Bits
#. button: Generate Source Code to Output


assemble output (asm)
----------------------------------------
#. Window > PIC Memory Views > Program Memory
#. output to file


========================================
sim30
========================================

ref:
DS70094
16-BIT LANGUAGE TOOLS GETTING STARTED

$ which sim30
/opt/microchip/xc16/v1.21/bin/sim30

.. sourcecode:: bash
    $ sim30 <instruction.txt>
    $ cat <instruction.txt>
    LD <target mpu>
    LC <test.out>
    IO NULL <output.txt>
    RP  #NOTE: reset
    E   #NOTE: execute
    quit


 LD dspic30super
    




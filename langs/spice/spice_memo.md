spice
====

* gournd node is 0

units

t   tera
g   giga
mega mega
k   kilo

m mili
u micro
p pico




## materials

R + - Ohm
C + - F
L + - H
D p n model    diode
Q C B E model  transistor
J D G S model  J-FET
M D G S Bulk model MOSFET

V + - DC V
V + - AC V [phase]
V + - sin(0V 141V 50Hz) （交流電圧源 offset 0V，amp freq振幅141V，f=50Hz）
V + - pulse(0V 10V 1ms 2ms 4ms 6ms 20ms)
（パルス電圧源=初期電圧0V, パルス電圧10V, 遅延時間1ms, 立上り2ms, 立下り4ms, パルス幅6ms, 周期20ms）
I は電流源であり，記述はVと同様

E + - + - gain  （電圧制御電圧源）
G + - + - トランスコンダクタンス （電圧制御電流源）
F + - name 制御素子名  gain     （電流制御電流源）
H + - name 伝達抵抗   （電流制御電圧源）


* comment
; comment

.model  Q1      NPN(ｘｘｘ ｘｘ ｘx）  （デバイスパラメタの定義）

.inc file （外部ファイルを（全て）読み込む）
.lib file （ライブラリから（モデルやサブサーキットなど必要情報を）読み込む）
.end                            （回路記述の終り）
.op                                     （DCバイアスの計算）

.dc target start end step
.dc V1 -3  10 0.5   （DC解析。V1を，-3V～10Vにわたって0.5Vづつ解析）

.ac dec num_per_dec start_Hz end_Hz
.ac dec 20 1k 10meg （AC解析。1kHz～10MHzを，10倍あたり20個所づつ解析）


.tran 1ns 100ns 20ns 0.1ns
（過渡解析。プリント出力1nsごと，最終時間100ns，20nsはプリント出力しない，時間刻み0.1ns以内）


## transient

.tran <step> <end>
.tran <step> <end> <start>
.tran <step> <end> <start> <tmax>

.tran 0.01m 2m

.four freq v(vout)


## output

print db(vout) ylabel dB ylimit -70 -10
print ph(vout) ylabel deg

$ ngspice -b -r dest.raw src.cir

setplot
plot v(in), v(out), v(4,5)


## opamp
use TL082.sub


## Elementary Device
### resistor
Rxxx n+ n- <resistance|r=>value <ac=val> <m=val> <scale=val> <temp=val> <dtemp=val> <tc1=val> <tc2=val> <noisy=0|1>

### capacitors
Cxxx n+ n- <value> <mname> <m=val> <scale=val> <temp=val> <dtemp=val> <tc1=val> <tc2=val> <ic=init_condition>

### Inductors
Lyyyy n+ n- <value> <mname> <nt=val> <m=val> <scale=val> <temp=val> <dtemp=val> <tc1=val> <tc2=val> <m=val> <ic=init_condition>

### coupled (mutual) inductors
form: Kxxx Lyyy Lzzz value

attributes:
* inductors: L? L?
* value: 0.4


### Switches
form:
* voltage controlled switch: Sxxx N+ N- NC+ NC- MODEL <ON><OFF>
* current controlled switch: Wyyy N+ N- VNAM MODEL <ON><OFF>


## Voltage and Current Sources

## Independent Sources for Voltage or Current
* form: VXXXXXXX N+ N− <<DC> DC/TRAN VALUE> <AC <ACMAG <ACPHASE>>> <DISTOF1 <F1MAG <F1PHASE>>> <DISTOF2 <F2MAG <F2PHASE>>>

* form: IYYYYYYY N+ N− <<DC> DC/TRAN VALUE> <AC <ACMAG <ACPHASE>>> <DISTOF1 <F1MAG <F1PHASE>>> <DISTOF2 <F2MAG <F2PHASE>>>


* Pulse General SPICE form: PULSE(V1 V2 TD TR TF PW PER)
* Sinusoidal General SPICE form: SIN(VO VA FREQ TD THETA)
* Exponential General SPICE form: EXP(V1 V2 TD1 TAU1 TD2 TAU2)
* Piece-Wise Linear General SPICE form: PWL(T1 V1 <T2 V2 T3 V3 T4 V4 ...>) <r=value> <td=value>
* Single-Frequency FM General SPICE form: SFFM(VO VA FC MDI FS)
* Amplitude modulated source (AM) General SPICE form: AM(VA VO MF FC TD)
* Transient noise source General SPICE form: TRNOISE(NA NT NALPHA NAMP RTSAM RTSCAPT RTSEMT)
* Random voltage source General SPICE form: TRRANDOM(TYPE TS <TD <PARAM1 <PARAM2>>>)


## Linear Dependent Sources

* Gxxxx: Linear Voltage-Controlled Current Sources (VCCS)
* Exxxx: Linear Voltage-Controlled Voltage Sources (VCVS)
* Fxxxx: Linear Current-Controlled Current Sources (CCCS)
* Hxxxx: Linear Current-Controlled Voltage Sources (CCVS)
* Non-linear Dependent Sources (Behavioral Sources)


## Active Devices

### Junction Diodes
form: DXXXXXXX n+ n− mname <area=val><m=val> <pj=val> <off> <ic=vd> <temp=val><dtemp=val>
n+ and n- are the positive and negative nodes. Personally I find it difficult to remember if the anode is positive or negative and visa-versa, so to make it easier for me, I rewrite the general form as:

DXXXXXXX anode cathode mname <area=val><m=val> <pj=val> <off> <ic=vd> <temp=val><dtemp=val>



### Bipolar Junction Transistors (BJTs)

form: QXXXXXXX nc nb ne <ns> mname<area=val> <areac=val> <areab=val> <m=val><off> <ic=vbe,vce> <temp=val> <dtemp=val>


### Junction Field-Effect Transistors (JFETs)

form: JXXXXXXX nd ng ns mname <area><off> <ic=vds,vgs> <temp=t>


### MESFETs

form: ZXXXXXXX ND NG NS MNAME <AREA><OFF> <IC=VDS, VGS>


### MOSFET devices

form: MXXXXXXX nd ng ns nb mname <m=val><l=val> <w=val> <ad=val> <as=val> <pd=val><ps=val> <nrd=val> <nrs=val> <off> <ic=vds,vgs, vbs> <temp=t>


### Sub Circuit
X?

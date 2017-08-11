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



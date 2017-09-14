VIVADO_HLS
====


```bash
$ /opt/Xilinx/Vivado_HLS/2015.4/bin/vivado_hls
```

click "create new project"

set "project name" and "location"

add remove C-based source files
    click "Next" (default setting. no change.)

add remove C-based testbench files
    click "Next" (default setting. no change.)

solution configuration1
    solution name: solution1
    clock period [ns] : 10
    part selection: xc7z010clg400-1
    click "finish"

"Project" > "New Source" > filename "multi\_apuint.cpp"

E [SIM-1] 'csim_design' failed: compilation error(s).
4
    while executing
"source /root/foo/Vivado_HLS/V_HLS_study_meeting/multi_apuint/solution1/csim.tcl"
    invoked from within
"hls::main /root/foo/Vivado_HLS/V_HLS_study_meeting/multi_apuint/solution1/csim.tcl"
    ("uplevel" body line 1)
    invoked from within
"uplevel 1 hls::main {*}$args"
    (procedure "hls_proc" line 5)
    invoked from within
"hls_proc $argv"

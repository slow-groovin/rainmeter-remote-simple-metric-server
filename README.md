# rainmeter-remote-simple-metric-server

A HTTP server exposes CPU, RAM, disk IO, and network IO metrics as **`Rainmeter`** remote data.

[中文README](.doc/README_ZH.md)

Files included:
- A Rust binary (release size: ~655KB, runtime memory ~9MB)
- An example Rainmeter skin configuration: `.doc/rainmeter-skin.ini`

## Overview
This project runs a tiny HTTP server (default: 0.0.0.0:3000) that exposes basic system metrics such as CPU usage, memory, disk IO (read/write rates), and network IO (rx/tx rates).
Metrics are printed as plain text, one metric per line, in the format key=[value], which makes it easy for tools like Rainmeter to parse using a regular expression.

Example output:
```
cpu=[15.7]
mem=[58]
swap=[0.0]
io_read=[0.0KB/s]
io_write=[0.0KB/s]
net_rx=[0.0KB/s]
net_tx=[0.0KB/s]
```


## Extending
Use "vibe coding" to customize and extend the exporter to fit your needs.
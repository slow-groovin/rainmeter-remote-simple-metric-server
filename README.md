# rainmeter-remote-simple-metric-server

A HTTP server exposes CPU, RAM, disk IO, and network IO metrics as **`Rainmeter`** remote data.

[中文README](.doc/README_ZH.md)

Files included:
- A Rust binary (release size: ~655KB, runtime memory ~9MB)
- An example Rainmeter skin configuration: `.doc/rainmeter-skin.ini`

## Installation

### 1. Install via Cargo

If you have Rust installed, simply run:

```sh
cargo install rainmeter-remote-simple-metric-server
```

### 2. Download from GitHub Releases

Visit the [GitHub Releases page](https://github.com/slow-groovin/rainmeter-remote-simple-metric-server/releases) and download the latest binary for your platform. Unzip and run the executable.

### (Optional) Use executable as a service
paste executable path and content of README to LLM, ask for steps to create a service for your OS. 


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


## Paramaters

You can configure the server using command-line arguments or environment variables. Command-line arguments take precedence.

### Command-line Arguments

- `--host`, `-h`         : Bind address (default: 0.0.0.0)
- `--port`, `-p`         : Port to listen on (default: 3000)
- `--path`               : HTTP path for metrics (default: /stats)
- `--interval`, `-i`     : Cache interval in seconds (default: 1)

Example:

```sh
rainmeter-remote-simple-metric-server --host 127.0.0.1 --port 8080 --path /metrics --interval 2
```

### Environment Variables

- `HOST`           : Bind address (default: 0.0.0.0)
- `PORT`           : Port to listen on (default: 3000)
- `PATH_STATS`     : HTTP path for metrics (default: /stats)
- `CACHE_INTERVAL` : Cache interval in seconds (default: 1)

Example:

```sh
HOST=127.0.0.1 PORT=8080 PATH_STATS=/metrics CACHE_INTERVAL=2 rainmeter-remote-simple-metric-server
```

## Extending
Use "vibe coding" to customize and extend the exporter to fit your needs.
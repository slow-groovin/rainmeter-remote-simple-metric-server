# rainmeter-remote-simple-metric-server
Rainmeter 远程HTTP数据源，暴露远程机器的 CPU、内存、磁盘 IO 和网络 IO 等指标

[English README](../README.md)

文件：
- 一个 Rust 可执行程序（release 可执行文件大小 **655KB**，运行内存约 **9MB**）
- 一个示例 skin 配置：`.doc/rainmeter-skin.ini`

## 安装

### 1. 通过 Cargo 安装

如果已安装 Rust，请运行：

```sh
cargo install rainmeter-remote-simple-metric-server
```

### 2. 从 GitHub Releases 下载

访问 [GitHub Releases 页面](https://github.com/slow-groovin/rainmeter-remote-simple-metric-server/releases)，下载与平台对应的最新二进制文件，解压后运行可执行文件。

### （可选）将可执行文件作为服务使用

把可执行文件的绝对路径和本README粘贴到大模型，让它针对你的操作系统给出具体创建服务的步骤。

## 说明
通过 HTTP 服务暴露远程机器的基础指标：CPU 使用率、内存、磁盘 IO（读写速率）和网络 IO（收发速率）。
这些指标以纯文本逐行输出，格式为 key=[value]，每行一个度量项，方便像 Rainmeter 这样的工具通过正则直接读取。

```
cpu=[15.7]
mem=[58]
swap=[0.0]
io_read=[0.0KB/s]
io_write=[0.0KB/s]
net_rx=[0.0KB/s]
net_tx=[0.0KB/s]
```

## 功能概述
- 在本地主机上开启一个轻量的 HTTP 服务（默认：0.0.0.0:3000）。
- 提供一个简单文本接口 `/stats` 返回当前系统的 CPU、内存、swap、IO 与网络流量速率。
- 缓存机制防止频繁刷新，可通过参数调整刷新间隔。

## 参数

可通过命令行参数或环境变量配置服务，命令行参数优先级更高。

### 命令行参数

- `--host`, `-h`         : 绑定地址（默认：0.0.0.0）
- `--port`, `-p`         : 监听端口（默认：3000）
- `--path`               : 返回度量的 HTTP 路径（默认：/stats）
- `--interval`, `-i`     : 缓存刷新间隔（秒，默认：1）

示例：

```sh
rainmeter-remote-simple-metric-server --host 127.0.0.1 --port 8080 --path /metrics --interval 2
```

### 环境变量

- `HOST`           : 绑定地址（默认：0.0.0.0）
- `PORT`           : 监听端口（默认：3000）
- `PATH_STATS`     : 返回度量的 HTTP 路径（默认：/stats）
- `CACHE_INTERVAL` : 缓存刷新间隔（秒，默认：1）

示例：

```sh
HOST=127.0.0.1 PORT=8080 PATH_STATS=/metrics CACHE_INTERVAL=2 rainmeter-remote-simple-metric-server
```


## 扩展方式
使用“vibe coding”方式按需定制扩展。
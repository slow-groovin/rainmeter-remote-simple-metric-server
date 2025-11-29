# rainmeter-remote-simple-metric-server
Rainmeter 远程HTTP数据源，暴露远程机器的 CPU、内存、磁盘 IO 和网络 IO 等指标

[English README](../README.md)

文件：
- 一个 Rust 可执行程序（release 可执行文件大小 **655KB**，运行内存约 **9MB**）
- 一个示例 skin 配置：`.doc/rainmeter-skin.ini`

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


## 扩展方式
使用“vibe coding”方式按需定制扩展。
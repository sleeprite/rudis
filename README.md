<div align="center">

<br />

<img src="./logo/logo.png" height="80"/>

<br />

[ROADMAP 2024](https://github.com/sleeprite/rudis/issues/11)

[Github](https://github.com/sleeprite/rudis) | [Gitee](https://gitee.com/Jmysy/rudis) | [Packages](./release) | [Docker](https://github.com/sleeprite/rudis/blob/master/docker/README.md) 

<a href='https://gitee.com/rudis/rudis/stargazers'><img src='https://gitee.com/rudis/rudis/badge/star.svg?theme=gvp' alt='star'/></a>
<a href="https://gitcode.com/rudis/rudis/stargazers"><img src="https://gitcode.com/rudis/rudis/star/badge.svg"/></a>
<a href="https://github.com/sleeprite/rudis"><img src="https://img.shields.io/github/stars/sleeprite/rudis?style=flat-square&logo=GitHub"/></a>
<a href="https://github.com/sleeprite/rudis/blob/master/LICENSE"><img src="https://img.shields.io/github/license/sleeprite/rudis.svg?style=flat-square"/></a>

<h4>高 性 能 内 存 数 据 库 </h4>

**[🔶 Explore the docs »](https://sleeprite.github.io/rudis)**

</div>

## 项目介绍

Rudis 是一个采用 Rust 语言编写得高性能键值存储系统，旨在利用 Rust 语言的优势来重新复现 Redis 的核心功能，以满足用户对高性能、可靠性和安全性的需求，同时保证与 Redis API 的兼容。

### 🌟 特性

- 跨平台，兼容 windows、linux、macos 系统。
- 兼容 字符串、集合、哈希、列表、有序集合数据结构。
- 提供 rdb 与 aof 机制以支持数据备份和恢复。
- 拥有卓越的处理速度和即时响应能力。
- 多个线程中并发创建和删除键值。
- 提供 Docker 部署方式。
- 兼容 RESP 协议规范。


## 快速入门


```
     /\_____/\
    /  o   o  \          Rudis 0.0.1
   ( ==  ^  == )
    )         (          Bind: 127.0.0.1:6379
   (           )
  ( (  )   (  ) )
 (__(__)___(__)__)

[2024-04-30T02:00:55Z INFO  rudis_server] Start loading appendfile
[=======================================] percent: 100% lines: 6/6
[2024-04-30T02:00:55Z INFO  rudis_server] Server initialized
[2024-04-30T02:00:55Z INFO  rudis_server] Ready to accept connections
```

### 普通安装

根据系统环境要求，[下载](./release) 匹配的 Rudis 版本

通过系统常规命令启动 Rudis 服务

```sh 
// windows 常规启动
start rudis-server.exe

// windows 配置文件启动
start rudis-server.exe --config ./config/rudis.conf

// windows 指定参数启动
start rudis-server.exe --port 6379
```

### 容器安装

通过 docker 容器启动 Rudis 服务

如需更多安装命令，请前往 [docker/README.md](./docker/README.md) 查看

```sh 
// docker 常规启动
docker run -p 6379:6379 ghcr.io/sleeprite/rudis:latest

// docker 指定参数启动
docker run -p 6379:8848 ghcr.io/sleeprite/rudis:latest --port 8848
```

## 配置说明

- 配置文件 (config): 指定Rudis配置文件路径。
- 绑定的主机地址 (bind): 指定Rudis服务器绑定地址。
- 端口 (port): Rudis服务器监听端口，默认6379。
- 密码 (password): 设置Rudis访问密码。
- 数据库数量 (databases): Rudis数据库数量，默认16。
- 数据持久化目录 (dir): RDB和AOF文件存储目录，默认"./"。
- 持久化日志路径 (appendfilename): AOF日志文件存储路径。
- 开启持久化 (appendonly): 是否开启AOF持久化。
- 数据文件名 (dbfilename): RDB持久化文件名，默认"dump.rdb"。
- 会话上限 (maxclients): 最大客户端连接数，默认1000。
- 定时任务频率 (hz): 定时任务执行频率，默认10次/秒。
- RDB保存策略 (save): 设置RDB自动保存条件。

## 网络架构

![alt text](./images/image.png)

## 项目结构

### cmds

Cmds 包是一个用 Rust 编写的模拟Rudis服务器的组件，主要负责实现Rudis协议的解析、数据库操作的执行以及相关结果的响应。该包内部包含了针对不同Rudis命令的实现，如SELECT、GET、SET等。其核心功能是根据Rudis协议规范，解析来自客户端的命令请求，并在模拟的Rudis数据库上执行相应的操作，再将结果返回给客户端。通过实现各个Rudis命令处理器，实现了对Rudis协议的完整支持，并提供了一个简单而有效的策略来处理不同类型的命令。

### persistence

Persistence 模块提供了 AOF（Append-Only File）和 RDB（Rudis Database） 两种持久化机制，它们共同确保了 Rudis 数据库的数据持久性和一致性。AOF 机制通过记录每个写操作并将它们追加到 AOF 文件中，实现了数据的持续更新和完整性。这种机制对于数据的准确性和可靠性至关重要，尤其是在系统故障或重启后能够确保数据的恢复。

### store

Store 包是一个基于内存的数据库管理系统。该模块提供了基础的数据结构约定，以及数据库操作功能，包括对数据的增、删、改、查等操作。

## 常用命令

echo 命令
```
127.0.0.1:6379> echo helloword
helloword
```

ping 命令
```
127.0.0.1:6379> ping
PONG
```

set 命令
```
127.0.0.1:6379> set user bailiang
OK
```

get 命令
```
127.0.0.1:6379> get user
bailiang
```

del 命令
```
127.0.0.1:6379> del username
(integer) 1
127.0.0.1:6379> del username password
(integer) 2
```

exists 命令
```
127.0.0.1:6379> exists user
(integer) 0
```

keys 命令
```
127.0.0.1:6379> keys *
(empty list or set)
```

auth 命令
```
127.0.0.1:6379> auth 123456
OK
```

expire 命令
```
127.0.0.1:6379> expire user 10000
(integer) 0
```

select 命令
```
127.0.0.1:6379> select 1
OK
```

dbsize 命令
```
127.0.0.1:6379> dbsize
(integer) 2
```

append 命令
```
127.0.0.1:6379> append user bailiang
(integer) 10
```

move 命令
```
127.0.0.1:6379> move user 0
OK
```

rename 命令
```
127.0.0.1:6379> rename username new_username
OK
```

## 构建源码

如果你希望通过构建源码的方式，得到发行包。

请使用 cargo 常用命令。

```
// 普通启动
cargo run

// 带参启动
cargo run -- --port 8848
cargo run -- --save 20/1 60/2

// 指定配置
cargo run -- --config rudis.conf

// 构建程序
cargo build

cargo build --release --target=x86_64-unknown-linux-musl

cargo build --release

// 代码检测
cargo clippy
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=sleeprite/rudis&type=Date)](https://www.star-history.com/#sleeprite/rudis&Date)

## 开源共建

Rudis 项目遵循 [GNU GENERAL PUBLIC LICENSE](https://github.com/sleeprite/rudis/blob/master/LICENSE) 开源协议，感谢这些优秀的 [Contributors](https://github.com/sleeprite/rudis/graphs/contributors)。

<a href="https://github.com/sleeprite/rudis/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sleeprite/rudis" />
</a>
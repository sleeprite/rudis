## 项目介绍

```
     /\_____/\
    /  o   o  \
   ( ==  ^  == )
    )         (
   (           )
  ( (  )   (  ) )
 (__(__)___(__)__)

Bind 127.0.0.1:6379;
Redis for Rust Server Started;
```

## 快速启动

```
// 普通启动
cargo run

// 带参启动
cargo run -- --password 123456

// 构建程序
cargo build
```

## 代码仓库

Github：https://github.com/sleeprite/redis-for-rust

Gitee：https://gitee.com/Jmysy/redis-for-rust

## 启动参数

- databases 数据库数量, 默认：16
- port 端口, 默认: 6379
- password 密码, 默认：None
- aof_file_path 持久化日志路径，默认：None

## 项目结构

### command

command 包是一个用Rust编写的模拟Redis服务器的组件，主要负责实现Redis协议的解析、数据库操作的执行以及相关结果的响应。该包内部包含了针对不同Redis命令的实现，如SELECT、GET、SET等。其核心功能是根据Redis协议规范，解析来自客户端的命令请求，并在模拟的Redis数据库上执行相应的操作，再将结果返回给客户端。通过实现各个Redis命令处理器，实现了对Redis协议的完整支持，并提供了一个简单而有效的策略来处理不同类型的命令。

### db

db 包是一个基于内存的数据库管理系统。该模块提供了基础的数据结构约定，以及数据库操作功能，包括对数据的增、删、改、查等操作。

### session

session 模块的设计目的是提供一个简单的会话管理功能，用于跟踪用户的操作状态，例如用户所选的数据库索引以及用户是否已认证等信息。这对于需要进行用户认证或者跟踪用户操作状态的系统是非常有用的。

### tools

tools 包是一个工具包，其中包含了一些通用的工具函数或工具类，用于辅助实现系统功能或处理特定任务。这些工具可以被其他模块或组件调用，以提高代码复用性和降低重复编写相似功能的工作量。

## 操作命令

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

set 命令 [过期]
```
127.0.0.1:6379> set user bailiang px 10000
OK
127.0.0.1:6379> set user bailiang ex 10
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

flushdb 命令
```
127.0.0.1:6379> flushdb
OK
```

flushall 命令
```
127.0.0.1:6379> flushall
OK
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

rpush 命令
```
127.0.0.1:6379> rpush key value1 value2
OK
```

lpush 命令
```
127.0.0.1:6379> lpush key value3 value4
OK
```

llen 命令
```
127.0.0.1:6379> llen key
(integer) 4
```

### 更新计划

- [ ] 持久存储，提供 aof 机制，存储每一条 update 命令到本地文件，并在启动时加载恢复停机前的状态；
- [ ] 日志体系，完善系统中日志打印，制订规范，帮助使用者更快速的追溯问题；
- [ ] 测试用例，针对项目中的每个命令，编写对应的单元测试，提高 Release 版本的稳定性；

### 开源共建

Redis for rust 项目遵循 [GNU GENERAL PUBLIC LICENSE](https://github.com/sleeprite/redis-for-rust/blob/master/LICENSE) 开源协议，感谢这些优秀的 [Contributors](https://github.com/sleeprite/redis-for-rust/graphs/contributors)。

<a href="https://github.com/sleeprite/redis-for-rust/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sleeprite/redis-for-rust" />
</a>

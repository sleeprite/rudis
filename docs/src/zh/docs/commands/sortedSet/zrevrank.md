# ZREVRANK

返回成员在有序集合中的**降序排名**（从 0 开始，分数最高者为 0）。

## 语法

```
ZREVRANK key member
```

## 返回值

- 成员存在：整数排名。
- 成员不存在或键不存在：`nil`。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZREVRANK myzset c
(integer) 0
redis> ZREVRANK myzset a
(integer) 2
redis> ZREVRANK myzset not-exist
(nil)
```

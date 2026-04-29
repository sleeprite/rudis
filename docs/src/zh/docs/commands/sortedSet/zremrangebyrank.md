# ZREMRANGEBYRANK

删除有序集合中**排名区间 `[start, stop]`** 内的所有成员（排名按分数升序，从 0 开始，支持负索引）。

## 语法

```
ZREMRANGEBYRANK key start stop
```

## 返回值

整数，被删除的成员数量。

> 当集合因此变为空时，键会被自动删除。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREMRANGEBYRANK myzset 0 1
(integer) 2
redis> ZRANGE myzset 0 -1 WITHSCORES
1) "c"    2) "3"
3) "d"    4) "4"
5) "e"    6) "5"
redis> ZREMRANGEBYRANK myzset -2 -1
(integer) 2
```

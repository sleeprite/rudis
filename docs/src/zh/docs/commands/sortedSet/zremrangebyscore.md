# ZREMRANGEBYSCORE

删除有序集合中分数在 `[min, max]` 区间内的所有成员。

## 语法

```
ZREMRANGEBYSCORE key min max
```

`min` / `max` 支持 `-inf` / `+inf` 和 `(value` 开区间，语法与 [`ZRANGEBYSCORE`](./zrangebyscore.md) 一致。

## 返回值

整数，被删除的成员数量。

> 集合因此变为空时，键会被自动删除。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREMRANGEBYSCORE myzset 2 4
(integer) 3
redis> ZRANGE myzset 0 -1 WITHSCORES
1) "a"    2) "1"
3) "e"    4) "5"
redis> ZREMRANGEBYSCORE myzset -inf +inf
(integer) 2
redis> EXISTS myzset
(integer) 0
```

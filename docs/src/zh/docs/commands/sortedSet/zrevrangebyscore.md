# ZREVRANGEBYSCORE

返回分数在区间内的所有成员，按分数**降序**。参数顺序是 `max` 在前、`min` 在后。

## 语法

```
ZREVRANGEBYSCORE key max min [WITHSCORES] [LIMIT offset count]
```

`max` / `min` 的语法与 [`ZRANGEBYSCORE`](./zrangebyscore.md) 一致：支持 `-inf` / `+inf` 和 `(value` 开区间。

## 返回值

符合条件的成员按降序排列的数组；带 `WITHSCORES` 则为扁平的 `[member, score, ...]`。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREVRANGEBYSCORE myzset +inf -inf
1) "e"
2) "d"
3) "c"
4) "b"
5) "a"
redis> ZREVRANGEBYSCORE myzset 4 2 WITHSCORES
1) "d"    2) "4"
3) "c"    4) "3"
5) "b"    6) "2"
redis> ZREVRANGEBYSCORE myzset +inf -inf LIMIT 0 3
1) "e"
2) "d"
3) "c"
```

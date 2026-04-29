# ZREVRANGE

按**分数降序**返回有序集合中指定排名区间内的成员。分数相同的成员按字典序降序排列。

## 语法

```
ZREVRANGE key start stop [WITHSCORES]
```

- `start` / `stop`：从 0 开始的降序索引，支持负数（`-1` 表示最后一个）。
- `WITHSCORES`：同时返回分数。

## 返回值

按降序排列的成员数组。加上 `WITHSCORES` 时为 `[member1, score1, member2, score2, ...]`。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREVRANGE myzset 0 -1
1) "e"
2) "d"
3) "c"
4) "b"
5) "a"
redis> ZREVRANGE myzset 0 -1 WITHSCORES
1) "e"    2) "5"
3) "d"    4) "4"
5) "c"    6) "3"
7) "b"    8) "2"
9) "a"    10) "1"
redis> ZREVRANGE myzset 0 2
1) "e"
2) "d"
3) "c"
```

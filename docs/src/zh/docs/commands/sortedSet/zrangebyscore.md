# ZRANGEBYSCORE

返回分数在 `[min, max]` 区间内的所有成员，按分数升序。

## 语法

```
ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]
```

- `min` / `max` 支持：
  - `-inf` / `+inf` — 负/正无穷
  - `3` — 闭区间边界（`score >= 3`）
  - `(3` — 开区间边界（`score > 3`）
- `WITHSCORES`：同时返回分数。
- `LIMIT offset count`：偏移与最大条数；`count = -1` 表示不限。

## 返回值

符合条件的成员按升序排列的数组；带 `WITHSCORES` 则为扁平的 `[member, score, ...]`。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZRANGEBYSCORE myzset 2 4
1) "b"
2) "c"
3) "d"
redis> ZRANGEBYSCORE myzset (2 (5
1) "c"
2) "d"
redis> ZRANGEBYSCORE myzset -inf +inf WITHSCORES LIMIT 1 2
1) "b"    2) "2"
3) "c"    4) "3"
```

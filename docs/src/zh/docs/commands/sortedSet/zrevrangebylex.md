# ZREVRANGEBYLEX

按成员的字典序**降序**返回区间内的成员。参数顺序为 `max` 在前、`min` 在后。

## 语法

```
ZREVRANGEBYLEX key max min [LIMIT offset count]
```

`max` / `min` 的语法与 [`ZRANGEBYLEX`](./zrangebylex.md) 一致（`-` / `+` / `[value` / `(value`）。该命令不支持 `WITHSCORES`。

## 返回值

字典序降序排列的成员数组。

## 示例

```
redis> ZADD lex 0 a 0 b 0 c 0 d 0 e 0 f
(integer) 6
redis> ZREVRANGEBYLEX lex + -
1) "f"  2) "e"  3) "d"  4) "c"  5) "b"  6) "a"
redis> ZREVRANGEBYLEX lex [d [b
1) "d"  2) "c"  3) "b"
redis> ZREVRANGEBYLEX lex (d (b
1) "c"
```

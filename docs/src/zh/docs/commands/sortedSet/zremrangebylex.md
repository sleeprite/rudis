# ZREMRANGEBYLEX

当集合所有成员分数相同时，删除字典序区间 `[min, max]` 内的所有成员。

## 语法

```
ZREMRANGEBYLEX key min max
```

`min` / `max` 支持 `-` / `+` / `[value` / `(value`，语法与 [`ZRANGEBYLEX`](./zrangebylex.md) 一致。

## 返回值

整数，被删除的成员数量。

> 集合因此变为空时，键会被自动删除。

## 示例

```
redis> ZADD lex 0 a 0 b 0 c 0 d 0 e
(integer) 5
redis> ZREMRANGEBYLEX lex [b [d
(integer) 3
redis> ZRANGE lex 0 -1
1) "a"
2) "e"
```

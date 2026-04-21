# ZRANGEBYLEX

当有序集合内所有成员**分数相同**时，按成员的**字典序**返回指定区间内的成员。

## 语法

```
ZRANGEBYLEX key min max [LIMIT offset count]
```

- `min` / `max` 支持：
  - `-` / `+` — 字典序负/正无穷
  - `[value` — 闭区间边界
  - `(value` — 开区间边界
- `LIMIT offset count`：偏移与最大条数，`count = -1` 表示不限。

> 该命令**不**支持 `WITHSCORES`。

## 返回值

字典序升序排列的成员数组。

## 示例

```
redis> ZADD lex 0 a 0 b 0 c 0 d 0 e 0 f
(integer) 6
redis> ZRANGEBYLEX lex - +
1) "a"  2) "b"  3) "c"  4) "d"  5) "e"  6) "f"
redis> ZRANGEBYLEX lex [b [d
1) "b"  2) "c"  3) "d"
redis> ZRANGEBYLEX lex (b (d
1) "c"
redis> ZRANGEBYLEX lex - + LIMIT 1 3
1) "b"  2) "c"  3) "d"
```

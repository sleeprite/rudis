# ZREVRANGEBYLEX

Returns members within a lexicographical range in **descending** order. Note the argument order: `max` first, `min` second.

## Syntax

```
ZREVRANGEBYLEX key max min [LIMIT offset count]
```

`max` / `min` use the same syntax as [`ZRANGEBYLEX`](./zrangebylex.md) (`-` / `+` / `[value` / `(value`). `WITHSCORES` is not supported.

## Return

Array of members in descending lexicographical order.

## Examples

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

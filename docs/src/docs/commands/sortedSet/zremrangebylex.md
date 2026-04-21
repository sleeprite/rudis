# ZREMRANGEBYLEX

When all members share the same score, removes all members within the lexicographical range `[min, max]`.

## Syntax

```
ZREMRANGEBYLEX key min max
```

`min` / `max` use the same syntax as [`ZRANGEBYLEX`](./zrangebylex.md): `-` / `+` / `[value` / `(value`.

## Return

Integer: number of members removed.

> When the sorted set becomes empty as a result, the key is removed automatically.

## Examples

```
redis> ZADD lex 0 a 0 b 0 c 0 d 0 e
(integer) 5
redis> ZREMRANGEBYLEX lex [b [d
(integer) 3
redis> ZRANGE lex 0 -1
1) "a"
2) "e"
```

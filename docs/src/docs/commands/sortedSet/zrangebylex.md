# ZRANGEBYLEX

When all members of the sorted set have the **same score**, returns the members within the given lexicographical range in ascending order.

## Syntax

```
ZRANGEBYLEX key min max [LIMIT offset count]
```

- `min` / `max` accept:
  - `-` / `+` — lex negative / positive infinity
  - `[value` — inclusive boundary
  - `(value` — exclusive boundary
- `LIMIT offset count`: offset and max count; `count = -1` means no limit.

> This command does **not** support `WITHSCORES`.

## Return

Array of members in ascending lexicographical order.

## Examples

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

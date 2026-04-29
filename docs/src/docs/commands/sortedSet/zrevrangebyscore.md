# ZREVRANGEBYSCORE

Returns all members whose score lies in the given range, in **descending** score order. Note the argument order: `max` first, `min` second.

## Syntax

```
ZREVRANGEBYSCORE key max min [WITHSCORES] [LIMIT offset count]
```

`max` / `min` use the same syntax as [`ZRANGEBYSCORE`](./zrangebyscore.md): `-inf` / `+inf` and `(value` for exclusive bounds.

## Return

Array of members in descending order; with `WITHSCORES`, a flat `[member, score, ...]` array.

## Examples

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

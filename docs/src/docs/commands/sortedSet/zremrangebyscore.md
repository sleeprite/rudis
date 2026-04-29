# ZREMRANGEBYSCORE

Removes all members whose score is in `[min, max]`.

## Syntax

```
ZREMRANGEBYSCORE key min max
```

`min` / `max` use the same syntax as [`ZRANGEBYSCORE`](./zrangebyscore.md): `-inf` / `+inf` and `(value` for exclusive bounds.

## Return

Integer: number of members removed.

> When the sorted set becomes empty as a result, the key is removed automatically.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREMRANGEBYSCORE myzset 2 4
(integer) 3
redis> ZRANGE myzset 0 -1 WITHSCORES
1) "a"    2) "1"
3) "e"    4) "5"
redis> ZREMRANGEBYSCORE myzset -inf +inf
(integer) 2
redis> EXISTS myzset
(integer) 0
```

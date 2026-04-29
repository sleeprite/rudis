# ZRANGEBYSCORE

Returns all members whose score lies in `[min, max]`, in ascending score order.

## Syntax

```
ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]
```

- `min` / `max` accept:
  - `-inf` / `+inf`
  - `3` — inclusive boundary (`score >= 3`)
  - `(3` — exclusive boundary (`score > 3`)
- `WITHSCORES`: also return the scores.
- `LIMIT offset count`: offset and maximum number of elements; `count = -1` means no limit.

## Return

Array of members in ascending order; with `WITHSCORES`, a flat `[member, score, ...]` array.

## Examples

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

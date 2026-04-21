# ZREVRANGE

Returns the specified rank range of members in a sorted set in **descending** score order. Members with equal scores are ordered by member name in reverse lexicographical order.

## Syntax

```
ZREVRANGE key start stop [WITHSCORES]
```

- `start` / `stop`: 0-based indices in the descending ordering, negative values are supported (`-1` means the last element).
- `WITHSCORES`: also return the scores.

## Return

Array of members in descending order; with `WITHSCORES`, flat `[member1, score1, member2, score2, ...]`.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREVRANGE myzset 0 -1
1) "e"
2) "d"
3) "c"
4) "b"
5) "a"
redis> ZREVRANGE myzset 0 -1 WITHSCORES
1) "e"    2) "5"
3) "d"    4) "4"
5) "c"    6) "3"
7) "b"    8) "2"
9) "a"    10) "1"
redis> ZREVRANGE myzset 0 2
1) "e"
2) "d"
3) "c"
```

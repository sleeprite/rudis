# ZPOPMAX

Pops and returns the member(s) with the **highest** scores from a sorted set, ordered descending.

## Syntax

```
ZPOPMAX key [count]
```

- `count` is optional, defaults to 1. A value of `0` pops nothing; a value larger than the set size pops everything. Negative values return an error.

## Return

A flat array `[member1, score1, member2, score2, ...]` in descending score order.

- Empty array when the key does not exist.
- When the sorted set becomes empty, the key is removed automatically.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZPOPMAX myzset
1) "c"
2) "3"
redis> ZPOPMAX myzset 2
1) "b"    2) "2"
3) "a"    4) "1"
redis> EXISTS myzset
(integer) 0
```

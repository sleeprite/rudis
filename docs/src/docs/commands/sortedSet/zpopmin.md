# ZPOPMIN

Pops and returns the member(s) with the **lowest** scores from a sorted set.

## Syntax

```
ZPOPMIN key [count]
```

- `count` is optional, defaults to 1. A value of `0` pops nothing; a value larger than the set size pops everything. Negative values return an error.

## Return

A flat array `[member1, score1, member2, score2, ...]` in ascending score order.

- Empty array when the key does not exist.
- When the sorted set becomes empty, the key is removed automatically.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZPOPMIN myzset
1) "a"
2) "1"
redis> ZPOPMIN myzset 100
1) "b"    2) "2"
3) "c"    4) "3"
redis> EXISTS myzset
(integer) 0
```

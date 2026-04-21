# ZSCAN

Incrementally iterates over members of a sorted set along with their scores, with optional glob-pattern filtering.

## Syntax

```
ZSCAN key cursor [MATCH pattern] [COUNT count]
```

- `cursor`: the starting point of this iteration. Pass `0` on the first call; pass the cursor returned by the previous call to continue. A returned value of `0` signals the end of iteration.
- `MATCH pattern`: only return members whose name matches the given glob pattern (`*`, `?`, `[...]`).
- `COUNT count`: maximum number of elements returned by this call. Defaults to `10`.

## Return

An array of two elements:

1. The next cursor. `0` means the iteration has finished.
2. A flat array: `[member1, score1, member2, score2, ...]`.

Returns `[0, []]` when the key does not exist, and an error when the key holds a value of a different type.

## Examples

```
redis> ZADD myzset 1 alpha 2 beta 3 gamma 4 delta
(integer) 4
redis> ZSCAN myzset 0
1) "0"
2) 1) "alpha"
   2) "1"
   3) "beta"
   4) "2"
   5) "gamma"
   6) "3"
   7) "delta"
   8) "4"
redis> ZSCAN myzset 0 MATCH alpha COUNT 100
1) "0"
2) 1) "alpha"
   2) "1"
redis> ZSCAN myzset 0 COUNT 2
1) "2"
2) 1) "alpha"
   2) "1"
   3) "beta"
   4) "2"
redis> ZSCAN myzset 2 COUNT 100
1) "0"
2) 1) "gamma"
   2) "3"
   3) "delta"
   4) "4"
```

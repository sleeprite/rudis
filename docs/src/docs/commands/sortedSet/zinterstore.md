# ZINTERSTORE

Computes the **intersection** of multiple sorted sets, combining scores according to the `AGGREGATE` strategy (optionally pre-multiplied by `WEIGHTS`), and stores the result in `destination`.

Source keys may also be plain `Set` values, in which case each member contributes a score of `1.0`.

## Syntax

```
ZINTERSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
```

- `destination`: target key; overwritten on non-empty result, not created (and any pre-existing value removed) on empty result.
- `numkeys`: number of source keys; must be `>= 1`.
- `WEIGHTS`: one multiplier per source key (length must equal `numkeys`). Defaults to all `1.0`.
- `AGGREGATE`: `SUM` (default), `MIN` or `MAX`.

The intersection is empty whenever **any source key does not exist** or the sources share no members.

## Return

Integer reply: the number of members stored in `destination`.

## Examples

```
redis> ZADD zia 1 a 2 b 3 c
(integer) 3
redis> ZADD zib 10 b 20 c 30 d
(integer) 3
redis> ZINTERSTORE dest 2 zia zib
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "12"
3) "c"    4) "23"
redis> ZINTERSTORE dest 2 zia zib AGGREGATE MIN
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "2"
3) "c"    4) "3"
redis> ZINTERSTORE dest 2 zia zib WEIGHTS 2 0.5
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "9"
3) "c"    4) "16"
```

### Missing source key → empty result & dest removed

```
redis> ZADD dest 99 leftover
(integer) 1
redis> ZINTERSTORE dest 2 zia missing
(integer) 0
redis> EXISTS dest
(integer) 0
```

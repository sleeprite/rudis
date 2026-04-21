# ZUNIONSTORE

Computes the **union** of one or more sorted sets, combining the scores of members that appear in multiple inputs according to the `AGGREGATE` strategy (optionally pre-multiplied by `WEIGHTS`), and stores the result in `destination`.

Source keys may also be plain `Set` values, in which case each member contributes a score of `1.0`.

## Syntax

```
ZUNIONSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
```

- `destination`: target key for the result. Overwritten when non-empty; not created (and any pre-existing value is removed) when the union is empty.
- `numkeys`: the number of source keys that follow; must be `>= 1`.
- `WEIGHTS`: a multiplier per source key (length must equal `numkeys`). Defaults to all `1.0`.
- `AGGREGATE`: how to combine scores for members appearing in more than one source: `SUM` (default), `MIN` or `MAX`.

## Return

Integer reply: the number of elements stored in `destination`.

## Examples

```
redis> ZADD zsa 1 a 2 b 3 c
(integer) 3
redis> ZADD zsb 10 b 20 c 30 d
(integer) 3
redis> ZUNIONSTORE dest 2 zsa zsb
(integer) 4
redis> ZRANGE dest 0 -1 WITHSCORES
1) "a"    2) "1"
3) "b"    4) "12"
5) "c"    6) "23"
7) "d"    8) "30"
redis> ZUNIONSTORE dest 2 zsa zsb AGGREGATE MAX
(integer) 4
redis> ZRANGE dest 0 -1 WITHSCORES
1) "a"    2) "1"
3) "b"    4) "10"
5) "c"    6) "20"
7) "d"    8) "30"
redis> ZUNIONSTORE dest 2 zsa zsb WEIGHTS 2 0.5
(integer) 4
```

### Mixing with a plain Set

```
redis> ZADD zs1 5 a 5 b
(integer) 2
redis> SADD set1 b c
(integer) 2
redis> ZUNIONSTORE dest 2 zs1 set1
(integer) 3
redis> ZRANGE dest 0 -1 WITHSCORES
1) "c"    2) "1"
3) "a"    4) "5"
5) "b"    6) "6"
```

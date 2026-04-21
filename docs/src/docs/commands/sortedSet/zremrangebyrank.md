# ZREMRANGEBYRANK

Removes all members within the **rank range `[start, stop]`** (ranks are ascending by score, 0-based; negative indices supported).

## Syntax

```
ZREMRANGEBYRANK key start stop
```

## Return

Integer: number of members removed.

> When the sorted set becomes empty as a result, the key is removed automatically.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c 4 d 5 e
(integer) 5
redis> ZREMRANGEBYRANK myzset 0 1
(integer) 2
redis> ZRANGE myzset 0 -1 WITHSCORES
1) "c"    2) "3"
3) "d"    4) "4"
5) "e"    6) "5"
redis> ZREMRANGEBYRANK myzset -2 -1
(integer) 2
```

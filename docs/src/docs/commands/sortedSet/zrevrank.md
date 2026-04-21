# ZREVRANK

Returns the **descending** rank of a member in a sorted set (0-based; the member with the highest score has rank 0).

## Syntax

```
ZREVRANK key member
```

## Return

- Integer rank when the member exists.
- `nil` when the member or the key does not exist.

## Examples

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZREVRANK myzset c
(integer) 0
redis> ZREVRANK myzset a
(integer) 2
redis> ZREVRANK myzset not-exist
(nil)
```

# GEOHASH

Returns the Geohash representation of one or more members.

## Syntax

```
GEOHASH key member [member ...]
```

## Return

Array reply: Array of 11-character Geohash strings. Returns nil for non-existent members.

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing
(integer) 1
redis> GEOHASH cities beijing
1) "wx4g0s8q3v0"
```

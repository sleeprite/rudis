# GEORADIUSBYMEMBER

This command is like GEORADIUS, but the center point is determined by the given member instead of longitude and latitude.

## Syntax

```
GEORADIUSBYMEMBER key member radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT count] [ASC|DESC]
```

## Return

Array reply: List of members within the radius.

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 116.3972 39.9163 forbidden_city
(integer) 2
redis> GEORADIUSBYMEMBER cities beijing 10 km
1) "beijing"
2) "forbidden_city"
```

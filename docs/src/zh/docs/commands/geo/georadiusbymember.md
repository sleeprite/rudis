# GEORADIUSBYMEMBER

这个命令和 GEORADIUS 命令一样， 都可以找出位于指定范围内的元素， 但是 GEORADIUSBYMEMBER 的中心点是由给定的位置元素决定的， 而不是使用经度和纬度来决定中心点。

## Syntax

```
GEORADIUSBYMEMBER key member radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT count] [ASC|DESC]
```

## Return

Array reply: 在范围内的位置元素列表。

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 116.3972 39.9163 forbidden_city
(integer) 2
redis> GEORADIUSBYMEMBER cities beijing 10 km
1) "beijing"
2) "forbidden_city"
```

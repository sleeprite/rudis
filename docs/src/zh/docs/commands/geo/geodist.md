# GEODIST

返回两个给定位置之间的距离。如果两个位置之间的其中一个不存在， 命令返回 nil。

## Syntax

```
GEODIST key member1 member2 [m|km|mi|ft]
```

## Return

Bulk string reply: 距离值，单位为指定单位。不存在时返回 nil。

单位：m=米(默认)，km=千米，mi=英里，ft=英尺

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 121.4737 31.2304 shanghai
(integer) 2
redis> GEODIST cities beijing shanghai km
"1067.598163"
```

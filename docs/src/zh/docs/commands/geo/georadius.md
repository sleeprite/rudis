# GEORADIUS

以给定的经纬度为中心， 返回键包含的位置元素当中， 与中心的距离不超过给定最大距离的所有位置元素。

## Syntax

```
GEORADIUS key longitude latitude radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT count] [ASC|DESC]
```

## Return

Array reply: 在范围内的位置元素列表。

- WITHDIST: 同时返回距离
- WITHCOORD: 同时返回坐标
- WITHHASH: 同时返回 Geohash
- COUNT n: 最多返回 n 个
- ASC/DESC: 按距离排序

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 116.3972 39.9163 forbidden_city
(integer) 2
redis> GEORADIUS cities 116.3974 39.9093 5 km WITHDIST
1) 1) "beijing"
   2) "0.0000"
2) 1) "forbidden_city"
   2) "0.7821"
```

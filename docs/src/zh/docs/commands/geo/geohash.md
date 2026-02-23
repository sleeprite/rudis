# GEOHASH

返回一个或多个位置元素的 Geohash 表示。

## Syntax

```
GEOHASH key member [member ...]
```

## Return

Array reply: 11 字符的 Geohash 字符串数组，成员不存在时返回 nil。

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing
(integer) 1
redis> GEOHASH cities beijing
1) "wx4g0s8q3v0"
```

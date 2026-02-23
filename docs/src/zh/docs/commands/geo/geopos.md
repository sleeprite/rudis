# GEOPOS

从 key 里返回所有指定位置元素的位置（经度和纬度）。

## Syntax

```
GEOPOS key member [member ...]
```

## Return

Array reply: 坐标数组，每个元素为 [经度, 纬度] 或 nil（成员不存在时）。

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing
(integer) 1
redis> GEOPOS cities beijing
1) 1) "116.3974"
   2) "39.9093"
redis> GEOPOS cities beijing shanghai
1) 1) "116.3974"
   2) "39.9093"
2) (nil)
```

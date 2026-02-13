# GEOADD

将指定的地理空间位置（经度、纬度、名称）添加到指定的 key 中。

## Syntax

```
GEOADD key longitude latitude member [longitude latitude member ...]
```

## Return

Integer reply: 新添加的成员数量。

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 121.4737 31.2304 shanghai
(integer) 2
redis> GEOADD cities 113.2644 23.1291 guangzhou
(integer) 1
```

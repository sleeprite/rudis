# GEOADD

Adds the specified geospatial items (longitude, latitude, name) to the specified key.

## Syntax

```
GEOADD key longitude latitude member [longitude latitude member ...]
```

## Return

Integer reply: The number of newly added members.

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 121.4737 31.2304 shanghai
(integer) 2
redis> GEOADD cities 113.2644 23.1291 guangzhou
(integer) 1
```

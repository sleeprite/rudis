# GEOPOS

Returns the positions (longitude and latitude) of all the specified members from the key.

## Syntax

```
GEOPOS key member [member ...]
```

## Return

Array reply: Array of coordinates, each element is [longitude, latitude] or nil (when member does not exist).

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

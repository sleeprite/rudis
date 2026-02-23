# GEODIST

Returns the distance between two members. Returns nil if one of the members does not exist.

## Syntax

```
GEODIST key member1 member2 [m|km|mi|ft]
```

## Return

Bulk string reply: The distance in the specified unit. Returns nil if a member does not exist.

Units: m=meters (default), km=kilometers, mi=miles, ft=feet

## Examples

```
redis> GEOADD cities 116.3974 39.9093 beijing 121.4737 31.2304 shanghai
(integer) 2
redis> GEODIST cities beijing shanghai km
"1067.598163"
```

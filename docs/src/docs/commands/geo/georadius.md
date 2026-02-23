# GEORADIUS

Returns the members within the specified radius centered at the given longitude and latitude.

## Syntax

```
GEORADIUS key longitude latitude radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT count] [ASC|DESC]
```

## Return

Array reply: List of members within the radius.

- WITHDIST: Also return distance
- WITHCOORD: Also return coordinates
- WITHHASH: Also return Geohash
- COUNT n: Return at most n members
- ASC/DESC: Sort by distance

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

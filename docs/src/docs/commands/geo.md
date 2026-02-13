# Geo

Rudis supports geospatial indexing based on a multi-index system (BTreeMap spatial index + HashMap member index), independent of ZSet.

## Commands

- [GEOADD](/docs/commands/geo/geoadd) - Add geographic points
- [GEOPOS](/docs/commands/geo/geopos) - Get member coordinates
- [GEODIST](/docs/commands/geo/geodist) - Calculate distance between two points
- [GEORADIUS](/docs/commands/geo/georadius) - Range query
- [GEORADIUSBYMEMBER](/docs/commands/geo/georadiusbymember) - Range query centered on a member
- [GEOHASH](/docs/commands/geo/geohash) - Return Geohash string

## Notes

- Use ZREM to remove members from a Geo collection (Redis compatible)
- TYPE returns "zset" for Geo keys (Redis compatible)

# Geo 地理信息

Rudis 支持地理空间索引，基于多路索引系统（BTreeMap 空间索引 + HashMap 成员索引）实现，不依赖 ZSet。

## 支持的命令

- [GEOADD](/zh/docs/commands/geo/geoadd) - 添加地理点
- [GEOPOS](/zh/docs/commands/geo/geopos) - 获取成员坐标
- [GEODIST](/zh/docs/commands/geo/geodist) - 计算两点距离
- [GEORADIUS](/zh/docs/commands/geo/georadius) - 范围查询
- [GEORADIUSBYMEMBER](/zh/docs/commands/geo/georadiusbymember) - 以成员为中心的范围查询
- [GEOHASH](/zh/docs/commands/geo/geohash) - 返回 Geohash 字符串

## 说明

- 使用 ZREM 可删除 Geo 集合中的成员（Redis 兼容）
- TYPE 对 Geo 键返回 "zset"（Redis 兼容）

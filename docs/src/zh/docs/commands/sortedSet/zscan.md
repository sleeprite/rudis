# ZSCAN

基于游标增量迭代有序集合中的成员及其分数，支持按 glob 模式过滤。

## 语法

```
ZSCAN key cursor [MATCH pattern] [COUNT count]
```

- `cursor`：本次迭代起点；首次传 `0`，之后使用上一次返回的 cursor 继续。返回 `0` 表示迭代结束。
- `MATCH pattern`：仅返回名称匹配该 glob 模式的成员（`*`、`?`、`[...]`）。
- `COUNT count`：单次返回的元素数量上限，默认 `10`。

## 返回值

由两个元素组成的数组：

1. 下一次迭代的 cursor（字符串 / 整数）；为 `0` 表示已遍历完成。
2. 扁平数组：`[member1, score1, member2, score2, ...]`。

若键不存在，返回 `[0, []]`；键类型不是有序集合时返回错误。

## 示例

```
redis> ZADD myzset 1 alpha 2 beta 3 gamma 4 delta
(integer) 4
redis> ZSCAN myzset 0
1) "0"
2) 1) "alpha"
   2) "1"
   3) "beta"
   4) "2"
   5) "gamma"
   6) "3"
   7) "delta"
   8) "4"
redis> ZSCAN myzset 0 MATCH alpha COUNT 100
1) "0"
2) 1) "alpha"
   2) "1"
redis> ZSCAN myzset 0 COUNT 2
1) "2"
2) 1) "alpha"
   2) "1"
   3) "beta"
   4) "2"
redis> ZSCAN myzset 2 COUNT 100
1) "0"
2) 1) "gamma"
   2) "3"
   3) "delta"
   4) "4"
```

# ZPOPMAX

弹出并返回有序集合中**分数最高**的成员（可指定多个），按分数降序排列。

## 语法

```
ZPOPMAX key [count]
```

- `count` 可选，缺省为 1；传 `0` 不弹出；超过集合长度时弹出全部。`count` 不能为负。

## 返回值

扁平数组 `[member1, score1, member2, score2, ...]`，按分数降序排列。

- 键不存在：空数组。
- 集合因此为空：键被自动删除。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZPOPMAX myzset
1) "c"
2) "3"
redis> ZPOPMAX myzset 2
1) "b"    2) "2"
3) "a"    4) "1"
redis> EXISTS myzset
(integer) 0
```

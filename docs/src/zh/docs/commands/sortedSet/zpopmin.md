# ZPOPMIN

弹出并返回有序集合中**分数最低**的成员（可指定多个）。

## 语法

```
ZPOPMIN key [count]
```

- `count` 可选，缺省为 1；传 `0` 不弹出；`count` 超过集合长度时弹出全部。`count` 不能为负。

## 返回值

扁平数组 `[member1, score1, member2, score2, ...]`，按分数升序排列。

- 键不存在：空数组。
- 集合因此为空：键被自动删除。

## 示例

```
redis> ZADD myzset 1 a 2 b 3 c
(integer) 3
redis> ZPOPMIN myzset
1) "a"
2) "1"
redis> ZPOPMIN myzset 100
1) "b"    2) "2"
3) "c"    4) "3"
redis> EXISTS myzset
(integer) 0
```

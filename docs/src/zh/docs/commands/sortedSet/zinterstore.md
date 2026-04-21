# ZINTERSTORE

对多个有序集合求**交集**，按 `AGGREGATE` 策略合并分数（可选 `WEIGHTS` 加权），结果写入 `destination`。

源键也可以是普通 `Set`，此时每个成员的分数按 `1.0` 计。

## 语法

```
ZINTERSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
```

- `destination`：目标键；结果非空时覆盖已有内容，结果为空时不创建并删除已存在的同名键。
- `numkeys`：源键数量，必须 `>= 1`。
- `WEIGHTS`：每个源键一个乘数，长度须等于 `numkeys`，默认全 `1.0`。
- `AGGREGATE`：`SUM`（默认）/ `MIN` / `MAX`。

只要**任一源键不存在**或**没有公共成员**，交集即为空。

## 返回值

整数，写入 `destination` 中的成员数量。

## 示例

```
redis> ZADD zia 1 a 2 b 3 c
(integer) 3
redis> ZADD zib 10 b 20 c 30 d
(integer) 3
redis> ZINTERSTORE dest 2 zia zib
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "12"
3) "c"    4) "23"
redis> ZINTERSTORE dest 2 zia zib AGGREGATE MIN
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "2"
3) "c"    4) "3"
redis> ZINTERSTORE dest 2 zia zib WEIGHTS 2 0.5
(integer) 2
redis> ZRANGE dest 0 -1 WITHSCORES
1) "b"    2) "9"
3) "c"    4) "16"
```

### 任一源键缺失 → 结果为空 & 删除 dest

```
redis> ZADD dest 99 leftover
(integer) 1
redis> ZINTERSTORE dest 2 zia missing
(integer) 0
redis> EXISTS dest
(integer) 0
```

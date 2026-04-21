# ZUNIONSTORE

对一个或多个有序集合求**并集**，按 `AGGREGATE` 策略合并分数（可选 `WEIGHTS` 加权），并把结果写入 `destination`。

源键也可以是普通 `Set`，此时每个成员的分数按 `1.0` 计。

## 语法

```
ZUNIONSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
```

- `destination`：结果写入的目标键；若已存在会被覆盖，若结果为空则不会创建（同时删除已存在的同名键）。
- `numkeys`：后面紧跟的源键数量，必须 `>= 1`。
- `WEIGHTS`：为每个源键配置一个乘数（长度必须 = `numkeys`），默认全 `1.0`。
- `AGGREGATE`：合并同名成员分数的方式，`SUM`（默认）/ `MIN` / `MAX`。

## 返回值

整数，写入 `destination` 中的成员数量。

## 示例

```
redis> ZADD zsa 1 a 2 b 3 c
(integer) 3
redis> ZADD zsb 10 b 20 c 30 d
(integer) 3
redis> ZUNIONSTORE dest 2 zsa zsb
(integer) 4
redis> ZRANGE dest 0 -1 WITHSCORES
1) "a"    2) "1"
3) "b"    4) "12"
5) "c"    6) "23"
7) "d"    8) "30"
redis> ZUNIONSTORE dest 2 zsa zsb AGGREGATE MAX
(integer) 4
redis> ZRANGE dest 0 -1 WITHSCORES
1) "a"    2) "1"
3) "b"    4) "10"
5) "c"    6) "20"
7) "d"    8) "30"
redis> ZUNIONSTORE dest 2 zsa zsb WEIGHTS 2 0.5
(integer) 4
```

### 与普通 Set 混用

```
redis> ZADD zs1 5 a 5 b
(integer) 2
redis> SADD set1 b c
(integer) 2
redis> ZUNIONSTORE dest 2 zs1 set1
(integer) 3
redis> ZRANGE dest 0 -1 WITHSCORES
1) "c"    2) "1"
3) "a"    4) "5"
5) "b"    6) "6"
```

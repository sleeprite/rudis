# ZADD

The Redis Zadd command is used to add one or more member elements and their fractional values to an ordered set.

## Syntax

```
ZADD key score member [score member ...]
```

## Return

The number of new members successfully added does not include those that have been updated or already exist.
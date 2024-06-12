# LPOP

The Redis Lpop command is used to remove and return the first element of a list.

## Syntax

```
LPOP key [count]
```

## Return

The first element of the list. When the list key does not exist, return nil.
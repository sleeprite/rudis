# LLEN

The Redis Llen command is used to return the length of a list. If the list key does not exist, it is interpreted as an empty list and returns 0. If the key is not a list type, return an error.

## Syntax

```
LLEN key
```

## Return

The length of the list.
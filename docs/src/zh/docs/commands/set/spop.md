# SPOP

The Redis Spop command is used to remove one or more random elements of a specified key from a collection, and after removal, it returns the removed elements.

## Syntax

```
SPOP key [count]
```

## Return

The removed random element. When the set does not exist or is empty, return nil.
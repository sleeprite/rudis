# SADD

The Redis Sadd command adds one or more member elements to a collection, and member elements that already exist in the collection will be ignored.

If the set key does not exist, create a set that only contains the added elements as members.

When the set key is not a set type, an error is returned.

## Syntax

```
SADD key member [member ...]
```

## Return

The number of new elements added to the collection, excluding ignored elements.
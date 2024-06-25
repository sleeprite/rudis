# HDEL

The Redis Hdel command is used to delete one or more specified fields from the hash table key, and non-existent fields will be ignored.

## Syntax

```
HDEL key field [field ...]
```

## Return

The number of successfully deleted fields, excluding ignored fields.
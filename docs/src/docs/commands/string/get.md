# GET

Get the value of. If the key does not exist the special value is returned. An error is returned if the value stored at is not a string, because only handles string values.

## Syntax

```
GET key
```

## Return

Bulk string reply: the value of key, or nil when key does not exist.
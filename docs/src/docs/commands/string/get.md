# GET

Get the value of. If the key does not exist the special value is returned. An error is returned if the value stored at is not a string, because only handles string values.

## Syntax

```
GET key
```

## Return

Return the value of the key, and if the key does not exist, return nil. If the key is not of string type, an error is returned.
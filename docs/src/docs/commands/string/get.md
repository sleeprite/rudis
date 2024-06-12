# GET

The Rudis Get command is used to obtain the value of a specified key. If the key does not exist, return nil. If the value stored in the key is not of string type, an error is returned.

## Syntax

```
GET key
```

## Return

Return the value of the key, and if the key does not exist, return nil. If the key is not of string type, an error is returned.
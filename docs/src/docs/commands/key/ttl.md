# TTL

The Redis TTL command returns the remaining expiration time of the key in seconds.

## Syntax

```
TTL key
```

## Return

When the key does not exist, return -2. When the key exists but the remaining survival time is not set, return -1. Otherwise, return the remaining survival time of the key in seconds.
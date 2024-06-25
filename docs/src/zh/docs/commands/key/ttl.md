# TTL

Like TTL this command returns the remaining time to live of a key that has an expire set, with the sole difference that TTL returns the amount of remaining time in seconds while PTTL returns it in milliseconds.

## Syntax

```
TTL key
```

## Return

Integer reply: TTL in seconds, or a negative value in order to signal an error.

- The command returns -2 if the key does not exist.

- The command returns -1 if the key exists but has no associated expire.
# SET

The Rudis SET command is used to set the value of a given key. If the key has already stored other values, SET will overwrite the old value and ignore the type.

## Syntax

```
SET key value [NX | XX] [EX seconds | PX milliseconds ]
```

## Option

The SET command supports a set of options that modify its behavior:

- EX seconds -- Set the specified expire time, in seconds (a positive integer).
- PX milliseconds -- Set the specified expire time, in milliseconds (a positive integer).
- NX -- Only set the key if it does not already exist.
- XX -- Only set the key if it already exists.

## Return

Simple string reply: OK if SET was executed correctly.

Null reply: (nil) if the SET operation was not performed because the user specified the NX or XX option but the condition was not met.
---
title: Persistence
titleTemplate: Advance
description: Essential information to help you get set up with Tachiyomi.
---

# Persistence

How Redis writes data to disk.

Persistence refers to the writing of data to durable storage, such as a solid-state disk (SSD). Redis provides a range of persistence options. 

## How to use it

The append-only file is an alternative, fully-durable strategy for Rudis. It became available in version 1.0.0.

You can turn on the AOF in your configuration file:

```
appendonly=true
```

From now on, every time Rudis receives a command that changes the dataset (e.g. SET) it will append it to the AOF. When you restart Rudis it will re-play the AOF to rebuild the state.
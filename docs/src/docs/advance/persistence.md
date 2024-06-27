---
title: Persistence
titleTemplate: Advance
description: Essential information to help you get set up with Tachiyomi.
---

# Persistence

How Rudis writes data to disk.

Persistence refers to the writing of data to durable storage, such as a solid-state disk (SSD). Rudis provides a range of persistence options. 

## RDB

By default Rudis saves snapshots of the dataset on disk, in a binary file called dump.rdb. You can configure Rudis to have it save the dataset every N seconds if there are at least M changes in the dataset.

For example, this configuration will make Rudis automatically dump the dataset to disk every 60 seconds:

```
save=60
```

This strategy is known as snapshotting.

```
dbfilename=dump.rdb
```

By default, the data will be retained in the dump.rdb file in the Rudis installation directory, and you can configure and modify the location through dbfilename.

## AOF

The append-only file is an alternative, fully-durable strategy for Rudis. It became available in version 1.0.0.

You can turn on the AOF in your configuration file:

```
appendonly=true
```

From now on, every time Rudis receives a command that changes the dataset (e.g. SET) it will append it to the AOF. When you restart Rudis it will re-play the AOF to rebuild the state.

```
appendfilename=./data/appendonly.aof
```

The data will be persisted to the appendonly.aof file in the Rudis installation directory by default, and you can configure and modify the location through appendfilename.

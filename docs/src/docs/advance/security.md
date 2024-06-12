---
title: Secure
titleTemplate: Advance
description: Essential information to help you get set up with Tachiyomi.
---

# Security

Security model and features in Redis.

## Network security

Access to the Redis port should be denied to everybody but trusted clients in the network, so the servers running Redis should be directly accessible only by the computers implementing the application using Redis.

In the common case of a single computer directly exposed to the internet, such as a virtualized Linux instance (Linode, EC2, ...), the Redis port should be firewalled to prevent access from the outside. Clients will still be able to access Redis using the loopback interface.

Note that it is possible to bind Redis to a single interface by adding a line like the following to the rudis.properties file:

```
bind 127.0.0.1
```

Failing to protect the Redis port from the outside can have a big security impact because of the nature of Redis. For instance, a single FLUSHALL command can be used by an external attacker to delete the whole data set.
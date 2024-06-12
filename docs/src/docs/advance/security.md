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
bind=127.0.0.1
```

Failing to protect the Redis port from the outside can have a big security impact because of the nature of Redis. For instance, a single FLUSHALL command can be used by an external attacker to delete the whole data set.

## Authentication

The legacy authentication method is enabled by editing the redis.conf file, and providing a database password using the setting. This password is then used by all clients.requirepass

When the setting is enabled, Redis will refuse any query by unauthenticated clients. A client can authenticate itself by sending the AUTH command followed by the password.requirepass

```
password=12345
```

The goal of the authentication layer is to optionally provide a layer of redundancy. If firewalling or any other system implemented to protect Redis from external attackers fail, an external client will still not be able to access the Redis instance without knowledge of the authentication password.

Since the AUTH command, like every other Redis command, is sent unencrypted, it does not protect against an attacker that has enough access to the network to perform eavesdropping.
# Protocol Spec

## Network layer

A client connects to a Redis server by creating a TCP connection to its port (the default is 6379).

While RESP is technically non-TCP specific, the protocol is used exclusively with TCP connections (or equivalent stream-oriented connections like Unix sockets) in the context of Redis.

## RESP protocol description
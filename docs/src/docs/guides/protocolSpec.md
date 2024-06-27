# Protocol Spec

## Network layer

A client connects to a Rudis server by creating a TCP connection to its port (the default is 6379).

While RESP is technically non-TCP specific, the protocol is used exclusively with TCP connections (or equivalent stream-oriented connections like Unix sockets) in the context of Rudis.

## RESP protocol description

### Simple strings

Simple strings are encoded as a plus (+) character, followed by a string. The string mustn't contain a CR (\r) or LF (\n) character and is terminated by CRLF (i.e., \r\n).

Simple strings transmit short, non-binary strings with minimal overhead. For example, many Rudis commands reply with just "OK" on success. The encoding of this Simple String is the following 5 bytes:

```
+OK\r\n
```

When Rudis replies with a simple string, a client library should return to the caller a string value composed of the first character after the + up to the end of the string, excluding the final CRLF bytes.

To send binary strings, use bulk strings instead.
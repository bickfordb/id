# ID

UUID wrapper utility to help sharing IDs between code that uses SQLite/PostgreSQL and Protobufs

## Encoding

### SQLite

If the `sqlite` feature is enabled, values are encoded as blobs instead of the default text encoding.

### Protobuf

IDs are encoded as a protobuf message containing two 64 bit integer fields. C.F. proto/id.proto

# ID

UUID wrapper utility to help sharing IDs between [SQLite/PostgreSQL] <-> Protobuf <-> WASM environments

## Encoding

### SQLite

If the `sqlite` feature is enabled, values are encoded as blobs instead of the default text encoding.

### Protobuf

IDs are encoded as a message containing two 64 bit integer fields.

# ID

ID is a UUID newtype wrapper that enables sharing UUIDs between code that uses SQLite/PostgreSQL, Protobufs and JSON/YAML (serde)

## Encodings

### SQLite

If the `sqlite` feature is enabled, values are encoded as 16 byte blobs instead of the inefficient sqlx text encoding (36bytes).

### Protobuf

IDs are encoded as a protobuf message containing two 64 bit signed integer fields. Please see proto/id.proto

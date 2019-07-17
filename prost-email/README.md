# prost-email

Example crate that generates encoded protobuf bytes.

## Usage

Generate an example bytes protobuf message:

```bash
cargo run -- --seed 42 > example/email.pb
```

Generate a compiled fieldset file:

```bash
protoc -o example/email.fdset proto/email.proto
```

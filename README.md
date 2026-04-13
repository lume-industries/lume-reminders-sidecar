# lume-reminders-sidecar

Reads reminders from a local JSON file (configurable via REMINDERS_PATH env var).

Produces `RemindersPayload` payloads conforming to the VZGLYD sidecar channel ABI.

This sidecar is designed to be reusable. Any slide can depend on it via git and receive data payloads through the standard channel ABI.

## Poll Interval

Every 5 seconds (file watch).

## Payload Format

`RemindersPayload` serialized as JSON bytes.

## Environment Variables

| Variable | Description |
|---|---|
| `REMINDERS_PATH` | Path to reminders JSON file (default: /data/reminders.json) |

## Usage

Build the sidecar:

```bash
cargo build --target wasm32-wasip1 --release
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

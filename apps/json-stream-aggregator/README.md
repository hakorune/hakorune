# json-stream-aggregator

Streaming JSONL-style aggregation app for the phase-293x real-app lane.

The parser is intentionally narrow and deterministic. It accepts the fixture
shape used by this app:

```text
{"user":"ana","bytes":10,"ok":true}
```

The goal is to exercise string scanning, per-key aggregation, map state, and
stable report generation without adding compiler-side workarounds.

## Run

```bash
./target/release/hakorune --backend vm apps/json-stream-aggregator/main.hako
```

Expected tail:

```text
summary=ok
```

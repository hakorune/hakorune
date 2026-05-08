# 293x-023 JSON Stream Aggregator EXE Parity

- Status: Landed
- Scope: complete `apps/json-stream-aggregator` pure-first direct EXE parity
  without app-side workarounds or C shim app classifiers.
- Gate: JSON stream aggregator pure-first EXE exits with `summary=ok` /
  `Result: 0`.

## Decision

Keep same-module global/user-box call value ownership in MIR metadata.
The C shim remains a metadata reader:

- Global call route facts may publish concrete handle argument types to the
  target function params.
- Generic method route results such as `string_substring` publish their handle
  result type before a later global call consumes the value.
- Same-module emit only reads `value_types` / route facts and does not
  rediscover `JsonLine` or `StringHelpers` semantics by name.

## Landed Shape

The JSON stream path now proves:

```text
JsonStreamAggregator.ingestLine(line: StringBox)
  -> JsonLine.stringField(line, "user")
  -> JsonLine.intField(line, "bytes")
  -> JsonLine.boolField(line, "ok")
  -> StringHelpers.to_i64(line.substring(start, end))
```

Two metadata seams were fixed:

- `"user"` / `"bytes"` / `"ok"` global-call args publish `StringBox` facts to
  `JsonLine.*Field` target params, so key concat lowers as string concat.
- `RuntimeDataBox.substring` publishes `StringBox` result facts, so
  `StringHelpers.to_i64/1` receives a string handle instead of scalar `i64`.

## Guard Tests

Added regression coverage for:

```text
global call const StringBox arg -> target param StringBox -> copy chain
substring result -> copy -> global target param StringBox
```

This fixes the real blocker structurally in MIR metadata publication.

## Validation

```bash
cargo fmt
cargo test -q publishes
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/apps/json_stream_aggregator_vm.sh
bash tools/smokes/v2/profiles/integration/apps/json_stream_aggregator_exe_runtime_boundary.sh
NYASH_DISABLE_PLUGINS=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  tools/selfhost/selfhost_build.sh \
    --in apps/json-stream-aggregator/main.hako \
    --exe /tmp/jsonagg.exe
NYASH_DISABLE_PLUGINS=1 /tmp/jsonagg.exe
```

Expected EXE output includes:

```text
events=5
users=3
total_bytes=78
summary=ok
Result: 0
```

## Next

Continue the remaining `real_apps_exe_boundary_probe` apps one at a time.
The next compiler blocker should come from `binary-trees`, `mimalloc-lite`, or
`allocator-stress`, not from JSON stream aggregator.

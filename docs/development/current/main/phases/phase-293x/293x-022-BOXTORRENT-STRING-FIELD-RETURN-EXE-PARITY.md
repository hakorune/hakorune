# 293x-022 BoxTorrent String Field Return EXE Parity

- Status: Landed
- Scope: complete the BoxTorrent mini direct EXE path by carrying user-box
  string field return facts through MIR-owned route metadata.
- Gate: BoxTorrent mini pure-first EXE exits with `summary=ok` / `Result: 0`.

## Decision

Do not add BoxTorrent-specific or broad user-box semantics to the C shim.
The owner remains MIR route facts:

- User-box/generic route facts may refine scalar placeholder metadata into a
  concrete Box handle when the call site proves the public ABI shape.
- `user_box_value_box_name` can read StringBox origins from string constants,
  string substring route results, and route `result_origin_box` facts.
- Field return hints can flow from observed `birth` argument bindings into
  later user-box method return classification.

## Landed Shape

The BoxTorrent path now proves:

```text
BoxTorrentChunker.ingest/4
  -> BoxTorrentStore.put/1(data: StringBox)
  -> ContentChunk.birth(cid, data, alloc_handle)
  -> ContentChunk.data : StringBox
  -> BoxTorrentStore.readData/1 returns string_handle
  -> BoxTorrentManifest.materialize/1 uses string concat
```

The EXE IR for `BoxTorrentManifest.materialize/1` now lowers:

```text
%r23 = call i64 @"BoxTorrentStore.readData/1"(...)
%r50 = call i64 @nyash.string.concat_hh(i64 %r18, i64 %r23)
```

instead of treating `%r23` as scalar `i64`.

## Guard Test

`user_box_method_route_plan` now includes a regression fixture for:

```text
Store.put(data=i64 placeholder)
  caller passes StringBox
  ContentChunk.birth stores data
  Store.readData returns chunk.data as string_handle
```

This fixes the real blocker without app-side workaround code.

## Validation

```bash
cargo fmt --check
cargo test --release user_box_method_route_plan --lib
cargo test --release generic_method_route_plan::tests::core_routes::records_runtime_data_get_for_typed_object_mapbox_result_origin --lib
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
bash tools/build_hako_llvmc_ffi.sh
tools/smokes/v2/run.sh --profile integration --filter boxtorrent_mini_exe.sh --skip-preflight
tools/smokes/v2/run.sh --profile integration --filter json_stream_aggregator_exe_runtime_boundary.sh --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
NYASH_LLVM_DUMP_IR=/tmp/boxtorrent-final.ll \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  tools/selfhost/selfhost_build.sh \
  --in apps/boxtorrent-mini/main.hako \
  --exe /tmp/boxtorrent-final
NYASH_DISABLE_PLUGINS=1 /tmp/boxtorrent-final
```

Expected EXE output includes:

```text
roundtrip=true
summary=ok
Result: 0
```

## Next

Fix the json-stream-aggregator EXE runtime parity boundary. Keep
allocator-detail planning, such as `HakoAllocHandle` internals, separate unless
it reappears as the active EXE boundary.

# 293x-019 Typed Object Call Param / Return Storage Flow

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Owner seam: MIR `TypedObjectPlan`

## Problem

BoxTorrent direct EXE reached `BoxTorrentChunker.ingest`, but
`BoxTorrentManifest` could not get a `typed_object_plan`.

The missing facts were not app-specific:

- `Manifest.name` is initialized through a same-module method parameter flow:
  `main -> BoxTorrentChunker.ingest(...) -> new BoxTorrentManifest(name)`.
- `Manifest.root_id` is assigned from a same-module global returning a
  string-like value whose ABI carrier is still `i64`.

If TypedObjectPlan trusts only ABI-level `i64` value types, it turns string
handles into scalar storage and rejects the whole object layout.

## Decision

Keep C shim thin. MIR owns storage inference for typed object layout:

- propagate observed storage into same-module `Global` and known `Method`
  parameters;
- prefer value-origin instruction storage over ABI-level value type when
  inferring field storage;
- infer string-like `BinaryOp::Add` results as handle storage when either side
  is handle-like;
- inspect same-module global returns conservatively, accepting only a single
  consistent return storage.

No app-name matching and no C-side user-box semantic rediscovery.

## Result

`BoxTorrentManifest` now emits a runtime slot object plan:

```text
name        -> handle
chunk_ids   -> handle
total_bytes -> i64
root_id     -> handle
```

The BoxTorrent EXE boundary moved from `typed_object_plan_missing` to the next
real seam:

```text
BoxTorrentChunker.ingest/4:
  BoxTorrentManifest.birth/1    -> user_box_birth_body_unsupported
  BoxTorrentManifest.addChunk/2 -> user_box_method_body_unsupported
  BoxTorrentManifest.seal/0     -> user_box_method_body_unsupported
```

The first pure-first EXE blocker remains `BoxTorrentChunker.ingest`, because the
method body is broader than the current single-block user-box method route.

## Gates

```bash
cargo test --release typed_object_plan --lib
cargo build --release --bin hakorune
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

## Next

Expand route coverage for `BoxTorrentChunker.ingest` / typed user-box method
bodies without adding app workarounds.

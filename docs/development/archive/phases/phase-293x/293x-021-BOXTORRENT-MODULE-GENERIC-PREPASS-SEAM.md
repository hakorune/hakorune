# 293x-021 BoxTorrent Module-Generic Prepass Seam

- Status: Landed
- Scope: lower the BoxTorrent `firstChunkId` / `refCount` EXE boundary without
  adding app-specific C shim semantics.
- Gate: real-app EXE boundary probe remains a blocker probe, not EXE parity.

## Decision

The module-generic boundary is lowered by moving more truth into MIR-owned
metadata and by keeping the C shim as a metadata/dataflow consumer:

- `GenericMethodRoute` may carry `result_origin_box`.
- LoweringPlan JSON forwards `result_origin_box`.
- TypedObjectPlan storage inference can propagate nullable handle returns and
  same-module receiver origins through `RuntimeDataBox` method surfaces.
- The same-module EXE shim may propagate an already known typed-object binding
  across PHI/copy/get sites, but must not infer BoxTorrent semantics by name.

## Landed Shape

`BoxTorrentManifest.firstChunkId/0` now exposes an Array/Map get result origin
as `StringBox`, so the same-module method route can return `string_handle`.

`BoxTorrentStore.refCount/1` now consumes:

```text
me.chunks.get(cid) -> result_origin_box=ContentChunk
phi(chunk)         -> typed object binding propagation
chunk.ref_count    -> typed object field_get
```

`ContentChunk` now gets a runtime slot object plan:

```text
cid          handle
data         handle
ref_count    i64
alloc_handle handle
```

## Current Boundary

The direct EXE probe now advances past the module-generic prepass seam and stops
at the next method-route boundary:

```text
reason=mir_call_no_route
bname=BoxTorrentChunker mname=ingest
```

This is expected. `BoxTorrentChunker.ingest/4` still has an unsupported method
body because it contains nested calls such as `BoxTorrentStore.put/1`.

## Validation

```bash
cargo test --release typed_object_plan::storage_inference --lib
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
tools/selfhost/selfhost_build.sh --in apps/boxtorrent-mini/main.hako --mir /tmp/boxtorrent-293x021.mir.json
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

## Next

`293x-022`: expand the `BoxTorrentChunker.ingest/4` direct user-box method route.
Keep `HakoAllocHandle` typed-object planning as a later allocator-detail seam
unless it reappears as the active EXE boundary.

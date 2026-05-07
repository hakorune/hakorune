# 293x-020 BoxTorrent Ingest User-Box Method Body Route Expansion

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Owner seam: MIR route metadata

## Problem

`BoxTorrentChunker.ingest/4` stayed outside the direct user-box method route
because the body used broader real-app shapes than the first conservative
single-block method body route:

- multi-block loop/branch/phi/select body;
- same-module user-box calls returning handle values;
- `RuntimeDataBox` method calls whose receiver was actually a user-box method
  parameter (`store.put(...)`);
- Map/Array method calls through typed-object fields inside `BoxTorrentStore.put`.

The app code is idiomatic and should not grow EXE-specific workarounds.

## Decision

Keep ownership in MIR:

- user-box method body support now accepts the broader structured method body
  shape only when every call site already has a route fact;
- user-box method routes infer receiver box origins from same-module call
  argument observations, so an untyped method parameter can be recovered when
  callers consistently pass a user-box handle;
- user-box method routes can return void, scalar, string handle, or object
  handle through the same DirectAbi contract;
- generic Map/Array routes read typed-object field origins through PHI/value
  metadata and accept redundant receiver arguments for `has` / `get`.

No C shim path learns app-specific names. The C backend remains a route reader.

## Result

`BoxTorrentChunker.ingest/4` now records the previously invisible call:

```text
block 179 inst 21:
  BoxTorrentStore.put/1
  type_id=3
  reason=user_box_method_body_unsupported
```

Inside `BoxTorrentStore.put/1`, these generic routes are now visible:

```text
MapBox.has              -> map_contains_any
RuntimeDataBox.get      -> runtime_data_load_any receiver_origin=MapBox
RuntimeDataBox.set      -> map_store_any receiver_origin=MapBox
RuntimeDataBox.push     -> array_append_any receiver_origin=ArrayBox
```

The `Store.put` nested route ledger is no longer receiver recovery. It now
reaches the typed object plan / handle-return seam:

```text
ContentChunk.birth/3:
  reason=typed_object_plan_missing
```

`ContentChunk.alloc_handle` currently flows from allocator methods that can
return an object handle or null, so the next card should handle nullable
user-box/object-handle return storage structurally.

The current EXE boundary probe stops slightly earlier in backend prepass order.
The exact first blocker can be either of these known module-generic prepass
surfaces depending on traversal:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BoxTorrentManifest.firstChunkId/0
target_shape_blocker_symbol=BoxTorrentStore.refCount/1
```

## Gates

```bash
cargo test --release user_box_method_route_plan --lib
cargo test --release generic_method_route_plan --lib
cargo build --release --bin hakorune
tools/selfhost/selfhost_build.sh --in apps/boxtorrent-mini/main.hako --mir /tmp/boxtorrent-293x020-field-phi.mir.json
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

## Next

`293x-021`: lower the BoxTorrent module-generic prepass seam
(`firstChunkId` / `refCount`) first, then continue into nullable handle
return/storage for the allocator handle path.

# 293x-018 Typed Void Side-Effect Global Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: accept the BoxTorrent allocator seeding body shape without adding
  app-specific C shim classifiers.

## Decision

- `TypedObjectPlan` remains the field layout truth for user boxes.
- MIR owns `typed_global_call_void_side_effect`; C only reads the proof from
  `lowering_plan`.
- A void same-module helper may be direct when every side-effect call in its
  body is already route-backed and the body returns the normal void sentinel.
- Same-module user-box body support may call route-backed global/user-box
  targets; it must not rediscover target semantics from names.
- The C same-module prepass now reports the exact failed `block/instruction/op`
  for module function failures instead of collapsing all failures to
  `module_function`.

## Accepted Shape

Accepted in this card:

- `void` same-module global helper
- route-backed `ArrayBox.push` / `RuntimeDataBox.push` side effects
- typed-object field get/set inside route-backed same-module user-box bodies
- transitive same-module birth/body route refresh until metadata stabilizes
- C proof reader support for `typed_global_call_void_side_effect`

Rejected / deferred:

- arbitrary void bodies with unplanned calls
- dynamic field addition
- broad user-box `newbox` interpretation in C
- app-specific matching for `HakoAllocPage`, `HakoAllocHeap`, or BoxTorrent
- `BoxTorrentChunker.ingest` method lowering

## Boundary Movement

Before this card, BoxTorrent direct EXE stopped while trying to emit the
allocator birth chain:

```text
target_shape_blocker_symbol=HakoAllocHeap.birth/0
reason=module_generic_prepass_failed
```

After this card, pure-first reaches the next real app method seam:

```text
consumer=mir_call_user_box_birth_same_module_emit site=b116.i11 symbol=HakoAllocHeap.birth/0
consumer=mir_call_global_uniform_mir_emit site=b64.i29 symbol=HakoAllocPage.seedBlocks/0
first_block=0 first_inst=19 first_op=mir_call
bname=BoxTorrentChunker mname=ingest
reason=mir_call_no_route
```

This is still an EXE boundary probe, not BoxTorrent EXE parity.

## Fixture

- Rust unit fixture:
  `mir::global_call_route_plan::tests::void_side_effect::refresh_module_semantic_metadata_accepts_void_side_effect_array_push_body`
- Real-app smoke pin:
  `tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh`

## Gates

```bash
cargo test --release void_side_effect --lib
cargo test --release route_plan --lib
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Continue one route shape at a time from the new boundary. The next concrete
seam is `BoxTorrentChunker.ingest`, not allocator page seeding.

# 293x-025: same-module body-shape cleanup

- Status: Landed
- Date: 2026-05-08
- Lane: `phase-293x real-app bringup`

## Summary

This is a BoxShape cleanup after binary-trees EXE parity. It does not expand the
accepted app surface.

The goal is to keep the next mimalloc-lite / allocator-stress work from adding
more responsibility to user-box route planning or the C shim.

## Changes

- Moved same-module body-shape acceptance from
  `user_box_method_route_plan/body_shape.rs` into the neutral MIR owner
  `same_module_body_shape.rs`.
- Kept route planners as consumers of shared body-shape facts instead of making
  global-call planning depend on user-box method planning internals.
- Pinned `allocator-stress` to the same exact EXE boundary as `mimalloc-lite`:
  `first_op=field_get` and
  `target_shape_blocker_symbol=HakoAllocHeap.release/1`.
- Deduplicated same-module result box metadata publication in the C shim through
  a small helper.

## Boundary

- No new accepted route shape was added.
- No app-specific `HakoAllocHeap` lowerer or by-name shim branch was added.
- The C shim still reads MIR-owned route metadata and value metadata only.

## Gates

```bash
cargo fmt --check
cargo test -q refresh_module_global_call_routes_accepts_typed_object_handle_return --lib
cargo test -q refresh_module_user_box_method_routes_accepts_object_handle_method_target --lib
bash tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh
```

# 293x-027 MIMALLOC-ALLOCATOR-STRESS-EXE-PARITY

Status: Landed
Date: 2026-05-08

## Decision

`mimalloc-lite` and `allocator-stress` direct EXE parity are accepted as
real-app gates. The former `HakoAllocHeap.release/1` `field_get` boundary is no
longer a pinned unsupported shape.

## Blockers Cleared

- `HakoAllocHeap.release/1` stopped in the pure-first module-generic prepass
  because the untyped `handle` param flowed through `phi` / `copy` before
  `handle.page_id`.
- `allocator-stress` then exposed a same-module `.inc` PHI refinement bug:
  explicit `dst_type: "i64"` PHIs were reclassified as bool when their incoming
  value was a `0` / `1` constant.

## Implementation

- MIR-owned user-box method route planning now infers unknown method params from
  unique field-use sets and propagates known callee param box facts back to
  caller params through `copy` / same-param `phi` paths.
- The inference only targets `Unknown` params, so known receivers are not
  reclassified from field names.
- Same-module pure-first PHI refinement now respects explicit PHI type metadata
  and does not overwrite an explicit `i64` PHI with a bool type.
- `mimalloc_lite_exe.sh` and `allocator_stress_exe.sh` are now dedicated EXE
  parity smokes.
- `real_apps_exe_boundary_probe.sh` remains as a no-op unsupported-shape probe
  pin; there are no remaining real-app unsupported-shape pins in this suite.
- The MIR root facade allowlist now records the existing user-box method route
  refresh entry points as refresh orchestration exports.

## Gates

```bash
cargo test -q user_box_method_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/mimalloc_lite_exe.sh
bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh
bash tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh
tools/checks/dev_gate.sh quick
```

## Next

Move the active implementation lane from real-app EXE unblock to the
mimalloc-grade substrate ladder already locked by `293x-026`, starting with the
minimal capability-module / fixed-width integer rows. Keep app code idiomatic;
future blockers should still land as compiler/runtime seams with smoke pins.

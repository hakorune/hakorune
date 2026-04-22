---
Status: Active
Date: 2026-04-22
Scope: cleanup card for deleting the legacy `.inc` `array_rmw_window` analyzer after the MIR metadata route is canonical.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md
---

# 292x-98: Delete `array_rmw_window` C Analyzer

## Problem

`array_rmw_window` already has MIR-owned route metadata and landed evidence
pins `reason=mir_route_metadata`. The old C analyzer still remains as a
fallback:

- `analyze_array_rmw_window_candidate`
- `trace_array_rmw_window_candidate`
- fallback branch in `emit_generic_method_get_by_window_or_policy(...)`

Keeping it means `.inc` still owns route legality for a migrated family.

## Decision

Delete the raw MIR shape analyzer for `array_rmw_window`. `.inc` may keep only:

- metadata reader / field validation
- helper emission
- skip marking
- fail-fast on malformed metadata

Missing metadata may fall through to generic lowering for legacy JSON, but
route-required smokes must assert `reason=mir_route_metadata`.

## Acceptance

```bash
cargo test -q array_rmw_window
cargo test -q build_mir_json_root_emits_array_rmw_window_routes
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```

If no focused smoke exists yet, add one or run an equivalent route-trace check
that proves `array_rmw_window result=hit reason=mir_route_metadata` before
deleting the fallback.

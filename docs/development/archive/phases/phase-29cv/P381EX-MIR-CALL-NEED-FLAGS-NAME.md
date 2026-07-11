# P381EX MIR-Call Need Flags Name

Date: 2026-05-06
Scope: rename Stage0 runtime declaration need flags to match their current owner.

## Context

The MIR-call prepass records which runtime declarations are needed by lowered
constructor, method, global, and extern calls. The flag struct still used the
historical `GenericPureNeedFlags` name.

## Change

Renamed the struct to:

```text
MirCallNeedFlags
```

Updated the MIR-call need policy, MIR-call prepass, and prescan owner.

No behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q mir::global_call_route_plan::tests::void_sentinel
cargo test -q runner::mir_json_emit::tests::global_call_routes::void_sentinel
cargo test -q mir::global_call_route_plan::tests::void_logging
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The runtime declaration need flags now have the same MIR-call owner vocabulary
as the classifier and prepass that populate them.

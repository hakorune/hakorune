---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: broader compat keep/archive cleanup の first slice として、built-in `InstanceBox.getField/setField` keep を retired し、kernel `by_name` surface から explicit `InstanceBox` special-case を外す。
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/plugin/invoke.rs
  - crates/nyash_kernel/src/tests.rs
  - src/runtime/type_registry.rs
  - src/runtime/host_api/host_box_ops.rs
---

# P23: BYN-min5 InstanceBox Builtin Keep Retire

## Purpose

- remove the explicit `InstanceBox.getField/setField` special-case from kernel `by_name`
- keep `InstanceBox` field access owned by slot/runtime routes that already exist
- avoid widening surrogate or hook/registry residue while shrinking the visible compat surface

## Current Truth

1. `InstanceBox.getField` and `InstanceBox.setField` are already present in `TypeRegistry` slots
2. runtime host API already owns direct `InstanceBox` field access behavior
3. no current daily caller-proof requires the kernel `by_name` export to preserve a builtin `InstanceBox` branch
4. `crates/nyash_kernel/src/plugin/invoke/by_name.rs` no longer contains an explicit `InstanceBox` match arm
5. `crates/nyash_kernel/src/plugin/invoke/instance_fields.rs` is removed

## Acceptance

1. `cargo test -p nyash_kernel instancebox_by_name_ -- --nocapture`
2. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
3. `bash tools/checks/phase29cl_by_name_surrogate_archive_guard.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Next Exact Front

1. broader compat keep/archive cleanup beyond the FileBox family and built-in `InstanceBox` keep

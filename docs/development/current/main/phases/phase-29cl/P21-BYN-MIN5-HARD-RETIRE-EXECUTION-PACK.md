---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: positive `P9` readiness judgment の次 exact front として、remaining `by_name` compat residue を narrow execution slices で退役していく順序を固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P12-BYN-MIN5-FILEBOX-WRITE-COMPAT-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P16-BYN-MIN5-FILEBOX-WRITEBYTES-COMPAT-SHRINK.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - src/llvm_py/instructions/direct_box_method.py
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
  - src/backend/mir_interpreter/handlers/boxes_file.rs
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
---

# P21: BYN-min5 Hard-Retire Execution Pack

## Purpose

- turn positive `P9` readiness into narrow execution order
- start from the explicit `FileBox` compat residue, not from surrogate or hook/registry residue
- keep `1 blocker = 1 narrow slice`; do not mix multiple `FileBox` methods in one change by default

## Fixed Order

1. `FileBox.open` execution slice
   - first exact front
   - retire the explicit Python-side compat leaf and matching built-in keep only if direct-route proof stays green
2. `FileBox.read` and `FileBox.close` execution slices
   - open these only after `FileBox.open` is stable
   - keep them separate unless proof shows they must move together
3. `FileBox.readBytes` execution slice
   - keep last in the family because it still has active binary-route proof
4. broader compat keep/archive cleanup
   - only after the remaining visible `FileBox` residue is stable

## Current Truth

1. `P9` is positive today, so hard-retire readiness is open
2. remaining visible execution residue is the explicit `FileBox` compat helper plus the built-in `FileBox` keep branch
3. compiled-stage1 surrogate residue is archive-only proof residue and must not be reopened by this pack
4. hook/registry keep residue is a frozen exact keep set and must not be widened by this pack
5. the first exact slice is `FileBox.open`

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_boxcall_plugin_invoke_args`
2. `cargo test -p nyash_kernel filebox_ -- --nocapture`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
4. `bash tools/checks/phase29cl_by_name_surrogate_archive_guard.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Reopen Rule

Reopen this pack only when one of these becomes true.

1. a new daily caller appears on `by_name`
2. surrogate archive proof regresses and `by_name` becomes the only green path again
3. `FileBox` execution requires widening the compat helper set
4. docs stop making it clear that this is the first hard-retire execution pack after positive readiness

## Non-Goals

1. deleting `module_string_dispatch.rs`
2. deleting `hako_forward_bridge.rs`
3. opening `type_registry.rs` or `unified_dispatch.rs` as the first slice
4. mixing multiple `FileBox` methods into one catch-all retire patch

## Next Exact Front

1. `FileBox.open` direct-route execution slice

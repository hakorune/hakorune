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
2. `FileBox.open` is direct-route through `nyash.file.open_hhh`
3. `FileBox.read` is direct-route through `nyash.file.read_h`
4. `FileBox.close` is direct-route through `nyash.file.close_h`
5. `FileBox.readBytes` is direct-route through `nyash.file.read_bytes_h`
6. the explicit Python-side `FileBox` compat helper now has an empty allowlist
7. the `FileBox` family has no remaining built-in kernel `by_name` keep
8. `InstanceBox.getField/setField` built-in kernel `by_name` keep is retired
9. compiled-stage1 surrogate residue is archive-only proof residue and must not be reopened by this pack
10. hook/registry keep residue is a frozen exact keep set and must not be widened by this pack
11. `P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md` is landed
12. `P25-BYN-MIN5-CORE-BY-NAME-SURFACE-INVENTORY.md` is landed
13. `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md` is landed and confirms the parent router is still live while surrogates stay archive-only
14. `P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md` is landed with current result `still-live keep`
15. `P30-BYN-MIN5-MIRBUILDER-SOURCE-SEAM-INVENTORY.md` is landed with current result `still-live compat owner`
16. `P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md` is landed with current result `still-live compat owner`
17. `P32-BYN-MIN5-PROGRAM-JSON-LIVE-CALLER-INVENTORY.md` is landed with current result `.hako live/bootstrap callers = monitor-only / near-thin-floor`
18. `P33-BYN-MIN5-PROGRAM-JSON-SHELL-HELPER-INVENTORY.md` is landed with current result `helper trio is heterogeneous; first helper-local bucket = tools/hakorune_emit_mir.sh`
19. `P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md` is landed with current result `tools/hakorune_emit_mir.sh` stays live; first exact seam = generated selfhost builder runner path`
20. `P35-BYN-MIN5-EMIT-MIR-SELFHOST-RUNNER-SEAM-INVENTORY.md` is landed with current result `execution code landed; generated selfhost builder runner seam = near-thin-floor / monitor-only`
21. the next exact front is `P36-BYN-MIN5-SELFHOST-BUILD-HELPER-INVENTORY.md`

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

1. `P36-BYN-MIN5-SELFHOST-BUILD-HELPER-INVENTORY.md`

---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: by-name retirement で delete/cutover を進める前の acceptance と reopen rule を固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
---

# P2: By-Name Acceptance And Reopen Rule

## Acceptance

1. docs acceptance
   - `by_name` is documented as retire target, not final architecture
2. backend proof acceptance
   - launcher/stage1/backend proof is green without introducing a new daily caller on `by_name`
   - lock smoke: `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
   - launcher source lane no longer uses explicit `invoke_by_name_i64` for `build exe`; backend receiver literals are acceptable only while direct-known-box lowering keeps them off the generic by-name tail
3. caller acceptance
   - visible daily callers point to TypeBox ABI v2 / Core C ABI / thin backend boundary
   - upstream daily caller pack (`method_call.py`, VM/WASM name-resolution users) is either demoted or explicitly marked compat-only
4. surrogate acceptance
   - compiled-stage1 surrogates remain explicit archive-only proof residue only
   - direct caller proof stays green through launcher/stage1/backend routes
   - guard: `tools/checks/phase29cl_by_name_surrogate_archive_guard.sh`
5. hook/registry acceptance
   - `hako_forward_bridge.rs` / `hako_forward.rs` / `hako_forward_registry_shared_impl.inc` remain explicit compat-only keep owners only
   - the keep cluster may stay as a frozen exact set without blocking readiness
   - hook registration / try-call / fallback policy do not become a new daily caller owner
6. helper-lane acceptance
   - `tools/hakorune_emit_mir.sh` and `tools/selfhost/selfhost_build.sh` remain monitor-only under the accepted `phase-29ci` helper-local proofs
   - `tools/smokes/v2/lib/test_runner.sh` remains near-thin-floor / monitor-only under the accepted helper audit scope
   - helper-local fan-out does not reopen without a fresh exact seam

## Reopen Rule

Reopen this phase when any of these become true:
1. a new daily caller is added to `nyash.plugin.invoke_by_name_i64`
2. a compiled-stage1 surrogate becomes the only green path again
3. docs begin to describe `by_name` as the intended final dispatch model
4. backend-zero caller cutover stalls because `by_name` ownership is ambiguous
5. a fresh exact helper-local seam appears under the accepted shell-helper keep set
6. hard delete / broad internal removal explicitly resumes

## Not An Acceptance Owner

The following are not final acceptance owners:
1. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
3. `lang/c-abi/shims/hako_llvmc_ffi.c` dynamic fallback path

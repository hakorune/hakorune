---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: by-name retirement で delete/cutover を進める前の acceptance と reopen rule を固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
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
   - launcher source lane no longer imports `LlvmBackendBox` / `selfhost.shared.backend.llvm_backend` for `build exe`
3. caller acceptance
   - visible daily callers point to TypeBox ABI v2 / Core C ABI / thin backend boundary
   - upstream daily caller pack (`method_call.py`, VM/WASM name-resolution users) is either demoted or explicitly marked compat-only
4. surrogate acceptance
   - compiled-stage1 surrogates remain explicit temporary keeps only

## Reopen Rule

Reopen this phase when any of these become true:
1. a new daily caller is added to `nyash.plugin.invoke_by_name_i64`
2. a compiled-stage1 surrogate becomes the only green path again
3. docs begin to describe `by_name` as the intended final dispatch model
4. backend-zero caller cutover stalls because `by_name` ownership is ambiguous

## Not An Acceptance Owner

The following are not final acceptance owners:
1. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
3. `lang/c-abi/shims/hako_llvmc_ffi.c` dynamic fallback path

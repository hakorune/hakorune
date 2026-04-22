---
Status: Active
Date: 2026-04-23
Scope: Delete-probe `pure_compile_minimal_paths` path #1 and #2.
Related:
  - docs/development/current/main/phases/phase-292x/292x-111-pure-compile-minimal-paths-inventory-card.md
  - docs/development/current/main/phases/phase-292x/292x-113-mapbox-duplicate-receiver-unified-dispatch-card.md
  - docs/development/current/main/phases/phase-292x/292x-114-hako-ll-stack-overflow-predelete-card.md
  - docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_minimal_paths.inc
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-112: Pure Compile Minimal Ret/Branch Deletion

## Goal

Remove the first two raw MIR recognizers from
`hako_llvmc_ffi_pure_compile_minimal_paths.inc`:

- path #1: single-block `const* -> ret const`
- path #2: const compare branch with two const arms and merge ret

These are language/control-flow shapes. The C boundary must not own their
legality. If they are still required, the owner must be generic lowering or a
MIR/Hako LL route, not another `.inc` pattern matcher.

## Non-Goals

- Do not edit the MapBox / ArrayBox minimal paths in this card.
- Do not edit the StringBox const-fold paths in this card.
- Do not add new metadata unless deletion proves impossible.

## Implementation

1. Delete the path #1 and #2 recognizer blocks.
2. Rebuild the C FFI.
3. Run ret-const, compare-branch, pure keep, historical ternary, and llvmlite
   monitor canaries.
4. If all pass, prune `hako_llvmc_ffi_pure_compile_minimal_paths.inc` in
   `tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv` to the guard's new
   debt count.

## Retry State

`292x-114` cleared the predelete blocker:

- `phase29x_backend_owner_daily_ret_const_min.sh`: PASS.
- `compat/llvmlite-monitor-keep`: PASS 3/3.
- Remaining `.inc` guard baseline before retry: 5 files / 47 analysis-debt
  lines.

Retry this card by deleting path #1/#2 again and pruning the allowlist if the
guard moves to the new lower count.

## Previous Probe Result

The 2026-04-23 deletion probe did not land.

- Deleting path #1/#2 locally reduced the thin-shim guard from 47 to 34 debt
  lines, but the `.inc` deletion was restored because the required canaries
  were not green.
- `archive/pure-historical` and `compat/pure-keep` pass. The
  `pure-historical` runner root fix landed separately as a smoke-only commit.
- The first failed canaries exposed a Rust VM duplicate-receiver bug for
  `MapBox.set/get`; that predelete bug is fixed in `292x-113`.
- After `292x-113`, `phase29x_backend_owner_daily_ret_const_min.sh` and
  `compat/llvmlite-monitor-keep` move past `route_profile` missing and abort
  with stack overflow.

The next retry can start from the same deletion diff because
`292x-114-hako-ll-stack-overflow-predelete-card.md` is now closed.

## Acceptance

```bash
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_ret_const_min.sh
bash tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected guard movement if deletion lands:

```text
hako_llvmc_ffi_pure_compile_minimal_paths.inc: 40 -> lower
```

---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P220a, LowerIfCompareFoldBinInts resolve-side text ABI
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P212A-FOLD-BIN-INTS-SCALAR-ROUTE-CLEANUP.md
  - docs/development/current/main/phases/phase-29cv/P219B-MIR-JSON-INST-NULL-GUARD-CLEANUP.md
  - lang/src/mir/builder/internal/lower_if_compare_fold_binints_box.hako
---

# P220a: Fold Bin Ints Resolve-Side Text ABI

## Problem

P219b advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerIfCompareFoldBinIntsBox._resolve_side_int/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`_resolve_side_int/3` is called from the string-producing `try_lower/1` path,
but its success branches return raw scalar integers while failure branches
return `null`.

## Decision

Keep this source-owned and mirror P212a:

```text
successful side resolution -> text handle
failure -> null sentinel
```

This also means the downstream MIR JSON emitter can see string-valued `const`
payloads for the canonical instruction `value` field. Consume the existing
P219a exact `mir_json_inst_field` proof accordingly:

```text
_emit_inst/1 key "value" -> string-or-void
```

Keep this exact to the MIR instruction schema proof. Do not add a new target
shape for i64/null resolver helpers.

## Non-Goals

- no `generic_string_body.rs` ABI expansion
- no `generic_i64_body.rs` expansion
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to compare-fold semantics
- no generic `RuntimeDataBox.get/1` widening

## Acceptance

Probe result should move past `_resolve_side_int/3`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p220a_resolve_side.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

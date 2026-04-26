---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow known-intrinsic comment inventory
Related:
  - src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
  - src/mir/control_tree/normalized_shadow/common/known_intrinsics.rs
  - docs/development/current/main/phases/phase-291x/291x-419-normalized-shadow-exit-reconnector-deprecated-stub-cleanup-card.md
---

# 291x-420: Normalized-Shadow Known-Intrinsic Comment Inventory

## Goal

Pick the next small compiler-cleanliness seam after exit-reconnector deprecated
stub cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

`expr_lowering_contract.rs` still documents `KnownIntrinsic` with stale
transition wording:

```text
method_name() and arity() methods REMOVED (deprecated)
```

The current shape is already clean:

```text
KnownIntrinsic             = marker enum
KnownIntrinsicRegistryBox  = metadata / lookup SSOT
```

There are no live `method_name()` or `arity()` methods to remove. The remaining
issue is only active-source wording.

## Decision

Clean only the `KnownIntrinsic` comment block in
`expr_lowering_contract.rs`.

Do not change:

- `KnownIntrinsic` variants
- `KnownIntrinsicRegistryBox`
- lookup behavior
- intrinsic allowlist contents

## Next Cleanup

`291x-421`: normalized-shadow known-intrinsic comment cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "REMOVED|deprecated|method_name\\(\\) and arity\\(\\)" \
  src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs
```

The final `rg` should produce no output.

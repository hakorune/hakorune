---
Status: Landed
Date: 2026-04-25
Scope: Clarify the constructor/birth compatibility contract before any generic-method `birth` prune attempt.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-216-receiver-surface-fallback-sunset-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-134-core-method-contract-inc-no-growth-guard-card.md
  - src/mir/builder/exprs.rs
  - src/mir/builder/calls/special_method_handlers.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-249 Constructor-Birth Compatibility Contract Card

## Goal

Make the remaining `birth` compatibility boundary explicit before any prune
attempt:

```text
NewBox(ArrayBox|MapBox)
  -> explicit birth() initializer marker
  -> generic-method `birth` compatibility emit-kind
  -> later constructor/birth carrier, if we decide to split ownership further
```

The current `birth` path is not a dead mirror. It still satisfies the
`NewBox→birth` runtime invariant in the MIR builder and keeps the ArrayBox /
MapBox literal lowering visible to downstream passes.

## Boundary

- Do not change lowering behavior.
- Do not prune the generic-method `birth` compatibility row in this card.
- Do not add new classifier growth.
- Do not collapse `NewBox` and `birth` into a single carrier here.
- Do not change the actual `nyash.array.birth_h` / `nyash.map.birth_h`
  constructor ABI in this card.

## Analysis

- `src/mir/builder/exprs.rs` emits an explicit `birth()` call after `NewBox`
  when lowering ArrayBox and MapBox literals.
- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc` still classifies
  `birth` as a transitional generic-method emit kind for zero-arg
  `ArrayBox` / `MapBox` compatibility.
- `lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc` still lowers
  that emit kind as a compatibility marker, not as a semantic owner.
- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc` already owns the
  concrete `nyash.*.birth_h` constructor ABI, so generic-method `birth` is
  not the only birth surface in the repo.
- The no-growth allowlist already names `birth` as a compatibility row whose
  deletion condition requires a constructor contract.

## Decision

Keep the generic-method `birth` row pinned for now.

The row is a compatibility initializer marker for zero-arg ArrayBox / MapBox
creation, not a standalone semantics owner. The next removal step, if any,
should come from a dedicated constructor/birth carrier or equivalent contract
split, not from receiver-row pruning.

## Result

The constructor/birth boundary is explicit enough to stay separate from the
receiver-surface cleanup lane. `birth` remains a compatibility row until the
carrier side is intentionally split.

## Next Work

The next follow-up is a constructor/birth carrier or metadata path, not
another receiver-surface prune.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

This card is docs-only. No lowering or runtime behavior changed.

---
Status: Landed
Date: 2026-04-25
Scope: Clarify why the remaining indexOf route-surface mirror cannot be pruned yet.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-197-mir-call-need-metadata-consumer-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-212 String IndexOf CoreMethod Carrier Design Card

## Goal

Prevent `indexOf` from being treated as prune-ready just because
`StringIndexOf` exists in the CoreMethod manifest.

## Current State

`StringIndexOf` is a manifest vocabulary row, but the backend route still uses
legacy surface state:

```text
method surface indexOf + runtime_string
  -> nyash.string.indexOf_hh
```

There is currently no `generic_method.indexOf` carrier emitted by MIR and no
route-kind/need consumer pair for `StringIndexOf`.

## Decision

Keep the MIR-call method-surface `indexOf` mirror row until a dedicated
StringIndexOf carrier lands.

The future implementation must be a separate card series with this order:

1. MIR route plan emits `generic_method.indexOf` only for StringBox-compatible
   receiver origins and supported arities.
2. The route metadata carries `core_method.op = StringIndexOf`, a String
   route-kind, helper symbol, value demand, and no-publication scalar result.
3. Route-policy and need-policy `.inc` consumers accept that metadata by enum
   tokens, not by rediscovering `mname == "indexOf"`.
4. Boundary fixtures and smokes prove metadata-bearing String indexOf without
   regrowing method-name classifiers.
5. Only then may the route-surface `indexOf` row be pruned.

## Non-Goals

- Do not add `generic_method.indexOf` in this card.
- Do not touch the compiler-state-heavy `indexOf` observer/window families.
- Do not conflate `StringBox.indexOf` with `ArrayBox.indexOf`; Array indexOf is
  a separate surface contract.
- Do not change helper symbols or String runtime behavior.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The no-growth allowlist now records the concrete `indexOf` deletion condition:
land a StringIndexOf CoreMethod route carrier plus route-policy and need-policy
metadata consumers before pruning the legacy method-surface row.

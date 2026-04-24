---
Status: Landed
Date: 2026-04-24
Scope: Pin the generic-method Set route/demand mirror with a focused guard.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-128-generic-method-policy-mirror-cut-card.md
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/src/runtime/collections/README.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/generic_method_set_policy_mirror_guard.sh
---

# 291x-129 Generic Method Set Policy Mirror Guard Card

## Goal

Close `291x-128` with the smallest BoxShape slice: make the `Set` route
vocabulary and `ArrayStoreString` demand contract explicit, then guard the
current `.hako` owner and C shim mirror against drift.

This card does not add a CoreBox row, parser rule, route shape, environment
variable, fallback, or broad generator.

## Decision

`ArrayStoreString` has this demand contract:

| Demand | Value | Reason |
| --- | --- | --- |
| `source_preserve` | `1` | The source string handle must survive until the slot store executes. |
| `identity_demand` | `0` | The store does not require stable object identity. |
| `publication_demand` | `1` | The lowering boundary publishes a public text handle for the array-string slot provider. |

`publication_demand` is a boundary/executor demand. It is not a second
stable-object identity policy.

## Implementation

- Updated `CollectionMethodPolicyBox.array_store_string_publication_demand(...)`
  to return `1` for `ArrayStoreString`.
- Added `tools/checks/generic_method_set_policy_mirror_guard.sh`.
- Wired the guard into `tools/checks/dev_gate.sh quick`.
- Updated `docs/tools/check-scripts-index.md` because a new
  `tools/checks/*.sh` entry was added.
- Documented the demand table in `lang/src/runtime/collections/README.md`.

## Guard Scope

The guard pins:

- `.hako` route string methods for the generic-method `Set` vocabulary
- C `GenericMethodSetRouteKind` enum names/order
- route names consumed by `.hako` `set_route(...)`
- route enums returned by C `classify_generic_method_set_route(...)`
- `ArrayStoreString` demand values on both sides
- `STORE_TEXT_PUBLIC` lowering for `ARRAY_STORE_STRING`
- runtime array-string route-state demand flags

The guard intentionally does not become a full generated policy table. That is
the later deletion target for the manual C mirror.

## Proof

```bash
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- Choose the next phase-291x cleanup card.
- Keep full generated generic-method policy metadata as a later, separate
  BoxShape slice.

---
Status: Closed task card
Date: 2026-04-24
Scope: Cut the first clean boundary for generic-method Set policy mirrors.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/README.md
---

# 291x-128 Generic Method Policy Mirror Cut Card

Closed by:

- `291x-129-generic-method-set-policy-mirror-guard-card.md`

## Goal

Turn the current `.hako` / `.inc` generic-method `Set` policy mirror into an
explicit task boundary before touching implementation.

This is a BoxShape / hygiene card. It does not add a CoreBox row, route shape,
parser rule, runtime behavior, environment variable, or fallback.

## Background

An audit found that `.hako` is close to being the policy owner, but the C shim
still mirrors collection method vocabulary and some route decisions manually.

The highest-risk local mirror is the `Set` path:

- `.hako` owner: `CollectionMethodPolicyBox.set_route(...)`
- C mirror: `GenericMethodSetRouteKind`
- C mirror classifier: `classify_generic_method_set_route(...)`
- C demand helpers: `classify_array_store_string_*`

The audit also found a drift candidate:

- `.hako` `array_store_string_publication_demand(...)` currently returns `0`.
- C `classify_array_store_string_publication_demand_publish_handle(...)`
  returns true for `ARRAY_STORE_STRING`.

This card does not decide which behavior is correct. It requires the next
implementation slice to make that decision explicit before changing code.

## Non-Goals

- rewriting `compiler_stageb.hako`
- generating the whole generic-method policy table
- changing `get` / `len` / `push` / `has` / `substring` lowering
- moving window analyzers out of C
- deleting transitional `.inc` files
- silently changing publication behavior

## Task Breakdown

1. Inventory the `Set` mirror vocabulary.
   - List every `.hako` route string returned by `CollectionMethodPolicyBox.set_route(...)`.
   - List every matching C enum in `GenericMethodSetRouteKind`.
   - Record any route with no opposite-side equivalent.

2. Inventory the `Set` classifier inputs.
   - `.hako`: `box_name`, `recv_origin`, `runtime_array_string`, `value_is_i64`, `value_is_string_handle`
   - C: `bname`, `plan->recv_org`, runtime array flags, `value_is_i64`, `value_is_string_handle`
   - Mark which inputs are true owner inputs and which are lowering-side mirrors.

3. Decide `ArrayStoreString` demand semantics.
   - Pick one explicit contract for source-preserve, identity-demand, and publication-demand.
   - If C must keep publishing at the boundary, rename/document it as a lowering boundary demand, not `.hako` policy truth.
   - If `.hako` should own the demand, update the C mirror to consume only the selected enum/table.

4. Add a small drift guard.
   - Prefer a text/table guard that checks route/demand names are synchronized.
   - Do not build a broad generator in this card.
   - The guard should fail when a new `Set` route is added on one side only.

5. Update shim docs.
   - Mark `hako_llvmc_ffi_generic_method_policy.inc` as a lowering-side mirror for `Set`.
   - State the deletion target: C classifier should eventually consume generated metadata/table only.

## Acceptance

The implementation card that closes this task must run:

```bash
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

It must also include one focused guard or smoke that pins the `Set` mirror
vocabulary and the `ArrayStoreString` demand decision.

## Next Implementation Card

Suggested name:

`291x-129 generic method set policy mirror guard`

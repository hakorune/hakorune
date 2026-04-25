---
Status: Complete (No Prune)
Date: 2026-04-25
Scope: Review whether the ArrayBox generic_method.push route-policy mirror row can be safely removed after 291x-243 metadata-first reordering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-243-push-route-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-176-push-emit-kind-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-246 Push Route Prune Review Card

## Goal

Determine whether the remaining ArrayBox generic_method.push route-policy mirror row is safely removable after 291x-243 made push routing metadata-first, while RuntimeDataBox.push fallback remains pinned.

## Context

After 291x-243 landed, two allowlist rows remain for push:
- `classify_generic_method_push_route box ArrayBox` (this review)
- `classify_generic_method_push_route box RuntimeDataBox` (explicitly pinned)

The route selection now prefers metadata flags (`runtime_array_push`, `runtime_array_string`) before the `bname` fallback:

```c
if (plan && (plan->runtime_array_push || plan->runtime_array_string)) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY;
}
// Fallback for metadata-absent cases
if (bname && !strcmp(bname, "ArrayBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY;
}
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_RUNTIME_DATA_APPEND_ANY;
}
```

## Evidence Review

### Route Policy vs Emit-Kind Distinction

This review examines the **route-policy** classifier (`classify_generic_method_push_route`), which is distinct from the emit-kind classifier that was rejected in 291x-176:

- **Emit-kind selection**: Determines if a method call is "push" operation
  - 291x-176 tried to prune this → REJECTED due to metadata-absent boundaries
  - Still has legacy fallback (not being removed)
  
- **Route selection**: Determines which helper symbol to use (array vs runtime_data)
  - 291x-243 made this metadata-first
  - This card reviews if the ArrayBox fallback can now be removed

### Prior Successful Prunes

Similar direct Box mirror rows were successfully pruned:
- 291x-240: MapBox get route-policy mirror removed
- 291x-241: MapBox has route-policy mirror removed

Pattern: Direct Box.method routes can be pruned when metadata covers all active boundaries.

### Known Metadata-Absent Boundaries

From 291x-176 rejection evidence:
- "metadata-absent direct ArrayBox and RuntimeData boundary fixtures still reach the legacy classifier"
- The deletion condition remains: `replace-with-arraypush-core-method-metadata-for-direct-arraybox-boundaries`

From 291x-243 design:
- "Fallback for metadata-absent cases"
- "Legacy box-name fallback remains for metadata-absent MIR."

### Fixture Coverage Check

The representative boundary fixture `s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh` includes ArrayPush with full CoreMethod metadata:

```json
"core_method": {
  "op": "ArrayPush",
  "proof": "core_method_contract_manifest",
  "lowering_tier": "cold_fallback"
}
```

However, this represents only **metadata-bearing** boundaries. The 291x-176 failure demonstrated that metadata-absent ArrayBox push boundaries still exist and require the fallback route.

## Analysis

The key question: Are there metadata-absent ArrayBox push calls that still reach route selection?

**Answer: YES**

Evidence:
1. 291x-243 explicitly designed the fallback for "metadata-absent cases"
2. 291x-176 rejection stated "metadata-absent direct ArrayBox...fixtures still reach the legacy classifier"
3. The deletion condition requires "metadata-absent-push-mutating-boundary-contract"
4. No such contract has been established between 291x-243 and this review

The route selection flow:
1. Emit-kind determines operation is "push" (may use metadata or fallback)
2. Route selection receives `bname` and `plan` 
3. If `plan` has metadata flags → use them (metadata-first path)
4. If `plan` is null or lacks flags → fall back to `bname` check

Metadata-absent cases enter route selection with `plan=null` or `plan` without the metadata flags, thus still requiring the `bname == "ArrayBox"` fallback.

## Decision

**No prune justified.**

The ArrayBox push route-policy mirror row should NOT be removed because:

1. **Metadata-absent boundaries still exist**: The emit-kind prune (291x-176) already demonstrated that metadata-absent ArrayBox push calls reach the backend
2. **Fallback is intentional**: 291x-243 explicitly kept the bname fallback for "metadata-absent cases"
3. **No new coverage contract**: No metadata-absent mutating boundary contract has been established since 291x-243
4. **Deletion condition unmet**: The allowlist explicitly requires "metadata-absent-push-mutating-boundary-contract" before removal

The route-policy reordering (291x-243) improved the preference order but did not eliminate the need for fallback. Removing the ArrayBox row would break metadata-absent direct ArrayBox push boundaries.

RuntimeDataBox.push fallback is already explicitly pinned and was out of scope for this review.

## Preservation Contract

The ArrayBox push route-policy mirror row must remain until:

1. A metadata-absent mutating boundary contract is established that covers all ArrayBox.push calls, OR
2. Proof is provided that no metadata-absent ArrayBox push boundaries exist in active fixtures/smokes

Neither condition is currently met.

## Result

- **No code changes made**
- **Allowlist rows preserved**: Both ArrayBox and RuntimeDataBox push route rows remain
- **Baseline unchanged**: `classify_generic_method_push_route` keeps both fallback branches
- **Review documented**: This card serves as the durable rationale for preservation

## Related Work

For future reference, the metadata-absent mutating boundary contract would need to:
- Cover direct ArrayBox.push(value) without CoreMethod metadata
- Cover RuntimeDataBox.push(value) facade calls
- Specify fallback behavior when route selection lacks metadata
- Maintain existing helper symbol contracts (`nyash.array.slot_append_hh`)
- Preserve return value shape (scalar_i64)
- Maintain alias/residence invalidation semantics

Such a contract is outside the scope of a simple route-policy mirror cleanup and would require its own dedicated design card.

## Acceptance

No changes to validate. Review complete.

```bash
bash tools/checks/current_state_pointer_guard.sh
# Confirm allowlist still has both push route rows:
grep "classify_generic_method_push_route" tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
```

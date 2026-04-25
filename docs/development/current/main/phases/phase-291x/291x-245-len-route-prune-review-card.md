---
Status: Review Complete - No Prune Justified
Date: 2026-04-25
Scope: Audit remaining generic_method.len route mirror rows (MapBox/ArrayBox bname fallback) after 291x-242 metadata-first reordering.
Related:
  - docs/development/current/main/phases/phase-291x/291x-242-len-route-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-169-metadata-absent-len-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-207-len-emit-kind-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-208-len-route-surface-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-245 Len Route Prune Review Card

## Goal

Determine whether the remaining `classify_generic_method_len_route` bname fallback rows are safely removable after 291x-242 reordered len route selection to prefer metadata flags first.

## Remaining Allowlist Rows

```text
lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc classify_generic_method_len_route box MapBox   1 core-box-contract replace-with-maplen-core-method-metadata-for-direct-mapbox-boundaries
lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc classify_generic_method_len_route box ArrayBox 1 core-box-contract replace-with-arraylen-core-method-metadata-for-direct-arraybox-boundaries
```

These correspond to lines 21-26 in `hako_llvmc_ffi_generic_method_len_policy.inc`:

```c
// Fallback for metadata-absent cases
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT;
}
if (bname && !strcmp(bname, "ArrayBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN;
}
```

## Audit Findings

### 1. Metadata-First Ordering Confirmed

After 291x-242, `classify_generic_method_len_route` checks metadata flags **before** bname fallback:

```c
if (plan->runtime_map_size) {          // lines 12-14: metadata FIRST
  return ...MAP_ENTRY_COUNT;
}
if (plan->runtime_array_len || plan->runtime_array_string) {  // lines 15-17
  return ...ARRAY_SLOT_LEN;
}
if (plan->runtime_string) {            // lines 18-20
  return ...STRING_LEN;
}
// ONLY THEN: bname fallback (lines 21-26)
```

### 2. "Metadata-Absent" Fixtures Actually Have Metadata

Investigation of the boundary fixtures used in 291x-169 smoke validation revealed:

```bash
$ jq '.functions[0].metadata.generic_method_routes' \
    apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json
[
  {
    "route_id": "generic_method.len",
    "box_name": "RuntimeDataBox",
    "method": "size",
    "receiver_origin_box": "MapBox",
    "core_method": {
      "op": "MapLen",
      "proof": "core_method_contract_manifest",
      ...
    }
  }
]
```

The "metadata-absent" RuntimeData boundary fixtures (used to validate the fallback contract in 291x-169) **include CoreMethod metadata**. They test RuntimeDataBox wrappers with metadata present, not truly metadata-absent MIR.

### 3. Truly Metadata-Absent Fixtures Don't Use Len

Survey of MIR fixtures without `generic_method_routes` metadata:

```bash
$ find apps/tests -name "*.mir.json" -exec sh -c \
  'if ! jq -e ".functions[0].metadata.generic_method_routes" "$1" >/dev/null 2>&1; then echo "$1"; fi' _ {} \;
```

Found 20+ prebuilt.mir.json and historical fixtures without metadata (bool_phi, user_box_*, sum_result_*, method_call_only_small, etc.).

**None of these call `.len()`, `.length()`, or `.size()` on MapBox or ArrayBox.**

### 4. Evidence of Unreachability

The bname fallback rows (lines 21-26) appear **unreachable** in current fixtures/smokes:

- Modern MIR JSON (post CoreMethod contract) includes `generic_method_routes` metadata
- Old fixtures without metadata don't use MapBox/ArrayBox len operations
- When metadata exists, metadata-first checks (lines 12-20) handle routing before bname fallback is reached

## Why No Prune Is Justified

### Conservative Policy Precedent

Phase-291x has consistently required **explicit proof of non-use** before removing fallback rows:

- **291x-169**: Rejected emit-kind len mirror prune because metadata-absent boundary fixtures failed
- **291x-164**: Pinned metadata-absent has fallback contract after rejected prune
- **291x-166**: Pinned metadata-absent get fallback contract after rejected prune
- **291x-184**: Pinned map storage-route fallback contract for metadata-absent coverage

The deletion conditions in the allowlist reflect this:

```text
replace-with-maplen-core-method-metadata-for-direct-mapbox-boundaries
replace-with-arraylen-core-method-metadata-for-direct-arraybox-boundaries
```

These require **both**:
1. CoreMethod metadata exists (✅ landed via 291x-167, 291x-168)
2. Metadata-absent boundary coverage exists (❌ not proven)

### Missing Evidence

To safely prune the bname fallback rows, we would need:

1. **Exhaustive fixture scan** proving no MIR JSON in the entire test corpus uses MapBox/ArrayBox len without metadata
2. **Producer contract** guaranteeing future MIR emitters will never produce metadata-absent len calls
3. **Fallback handling** for legacy MIR JSON that might exist outside the test corpus

None of these conditions are met.

### RuntimeData Compatibility Risk

The bname fallback provides a safety net for RuntimeDataBox len calls when metadata is missing. Removing it could break:

- Legacy MIR JSON in user projects
- Future RuntimeDataBox scenarios where metadata propagation fails
- Edge cases in MIR emitter that might not attach metadata

The cost of keeping the fallback (2 allowlist rows, 6 lines of C code) is minimal compared to the risk of breaking metadata-absent boundaries.

## Decision

**Do not prune the MapBox/ArrayBox len route mirror rows.**

The bname fallback rows are **defensive compat guards**, not dead code. They provide:

1. **Safety net** for metadata-absent MIR (even if none currently exists in fixtures)
2. **RuntimeData compat** for wrapper scenarios
3. **Producer independence** - MIR emitter doesn't need perfect metadata coverage

The allowlist rows should remain pinned with their current deletion conditions until:

- Explicit evidence proves no metadata-absent len boundaries exist in any MIR corpus
- Producer contract guarantees metadata presence
- Replacement fallback handling is in place

## Acceptance

No code changes. This is a review card.

```bash
# Verification that current guard baseline is unchanged
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Result

The two `classify_generic_method_len_route` allowlist rows remain required:

```text
classifiers=18 rows=18 (unchanged)
- classify_generic_method_len_route box MapBox
- classify_generic_method_len_route box ArrayBox
```

Future prune attempts must provide explicit proof of non-use or a documented replacement fallback strategy.

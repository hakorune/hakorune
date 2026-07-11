# 293x-370 METADATA-PROMOTE-001 Active Matrix Guard

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-001` hardens the MIR metadata catalog guard so the current
promotion matrix cannot silently lose its active rows, near-term rows,
metadata-only stop lines, or ordered follow-up task queue.

This is a BoxShape guard row only. It does not change MIR JSON shape, Rust
metadata structs, verifier behavior, backend lowering, or runtime behavior.

## Responsibility

The guarded SSOT remains:

```text
docs/reference/mir/metadata-facts-ssot.md
```

The guard owner is:

```text
tools/checks/mir_metadata_catalog_guard.sh
```

This row fixes the first task from the promotion queue:

```text
METADATA-PROMOTE-001:
  harden catalog rows for active contracts and routes without changing MIR JSON
  shape
```

## Guarded Surface

The guard now requires the catalog to keep:

- `Current Promotion Matrix`
- `Promote / Treat As Active Now`
- `Promote Soon / Prepare A Dedicated Row`
- `Keep As Metadata / Do Not Promote Directly`
- `Promotion Task Queue`

It also pins representative rows and stop lines for:

- active contracts/routes such as `lowering_plan`, `typed_object_plans`,
  `static_data_plans`, `effect_plans`, required `inline_plans`,
  `string_kernel_plans`, `placement_effect_routes`,
  `exact_numeric_runtime_check_contracts`, and hako_alloc packed-store verifier
  rows;
- near-term promotion rows such as packed materialization boundary,
  direct-read consumption, LoopRange, array/text routes, enum-use verification,
  and exact numeric operation routes;
- metadata-only rows such as raw declarations, raw facts, family-specific
  compatibility rows, and temporary seed routes.

## Stop Lines

- Do not use this guard as permission to promote a row in code.
- Do not combine promotion cleanup with allocator behavior rows.
- Do not promote seed routes to CorePlan ownership.
- Do not enable packed backend lowering without a proof-bearing route,
  capability gate, and `boxed_fallback=false` contract.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

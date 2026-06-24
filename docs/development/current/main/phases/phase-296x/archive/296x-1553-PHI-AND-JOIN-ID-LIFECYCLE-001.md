# 296x-1553 PHI-AND-JOIN-ID-LIFECYCLE-001

Status: landed
Date: 2026-06-22

## Purpose

Record the PHI and join_id lifecycle decision without opening route
selection or a nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps `CarrierVar.join_id` parked while the trim_helper,
promoted_body_locals, and merge_from ownership boundaries stay separate.

## Scope

```text
BoxCount: one consultation inventory
owner: PHI and join_id lifecycle
input: join_id producer inventory, PHI carrier inventory, trim helper inventory
output: one durable lifecycle inventory and guard
```

## Decision

```text
keep CarrierVar.join_id parked as test vocabulary until a production producer is named
keep PHI and join_id lifecycle separate from trim_helper, promoted_body_locals, and merge_from ownership
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_phi_and_join_id_lifecycle_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_phi_and_join_id_lifecycle_guard.sh
```

## Acceptance

```text
the PHI/join_id lifecycle space is fixed in one machine-readable fixture
join_id remains parked
route selection remains unopened
nightly rustc adapter remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```

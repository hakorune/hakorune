# 293x-381 METADATA-CATALOG-004 Post-Promotion Reconcile

Status: landed
Date: 2026-05-15

## Decision

`METADATA-CATALOG-004` is the next BoxShape cleanup row after `MIMAP-020A`.
It reconciles the metadata promotion docs after `METADATA-PROMOTE-001` through
`METADATA-PROMOTE-006` have landed.

This row is docs / guard cleanup only. It must not change MIR JSON shape, Rust
metadata structs, verifier behavior, backend lowering, runtime behavior, or any
allocator behavior route.

## Scope

- Mark the `METADATA-PROMOTE-001` through `METADATA-PROMOTE-006` queue as a
  landed promotion wave instead of an open queue.
- Add a small next-work ledger that separates future metadata docs cleanup from
  behavior rows.
- Update the phase taskboard cleanup sidecar so metadata catalog/promotion rows
  have the same landed visibility as the current-state tail.
- Keep allocator/provider activation, packed backend lowering, and seed-route
  deletion out of this row.

## Planned Task Order

| Step | Task | Output | Stop line |
| --- | --- | --- | --- |
| `004.1` | Reconcile `metadata-facts-ssot.md` promotion queue wording. | Completed wave plus future candidates. | no schema or behavior edits |
| `004.2` | Reconcile phase taskboard visibility. | `METADATA-CATALOG-003` and `METADATA-PROMOTE-001..006` are listed as landed cleanup rows. | no allocator row status changes |
| `004.3` | Run metadata/current guards. | Guard evidence recorded in this card. | no broad gate escalation unless docs guard requires it |

## Future Work Split Rules

Only split follow-up rows when a real owner appears:

| Candidate | Trigger | Boundary |
| --- | --- | --- |
| `METADATA-CONSUME-PLACEMENT-001` | A backend consumer is ready to stop reading family-specific placement rows directly. | One consumer family per row; proof-bearing route only. |
| `METADATA-SEED-RETIRE-001` | A generic route can replace one exact seed payload with the same proof and publication boundary. | One seed family per row; no broad seed deletion. |
| `PACKED-BACKEND-*` | Packed direct-read backend lowering is explicitly reopened. | Must carry backend proof, capability gate, and `boxed_fallback=false`. |
| verifier contract sidecar | A metadata row starts deciding fail-fast legality. | Promote the derived contract, not raw source attrs. |

## Stop Lines

- Do not combine metadata cleanup with allocator behavior rows.
- Do not promote seed payloads to CorePlan ownership.
- Do not enable packed record backend lowering.
- Do not reopen provider hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not turn Stage0 source attrs into layout, legality, optimizer, or backend
  decisions.

## Closeout

`METADATA-CATALOG-004` reconciles the stale open promotion queue after
`METADATA-PROMOTE-001` through `METADATA-PROMOTE-006` landed.

Changed docs:

- `docs/reference/mir/metadata-facts-ssot.md` now treats the promotion queue as
  a completed wave and keeps future work in owner-triggered row families.
- `293x-369-METADATA-CATALOG-003-PROMOTION-MATRIX.md` now records the original
  queue as landed history instead of future work.
- `293x-mimalloc-port-taskboard.md` exposes the full metadata catalog/promotion
  cleanup wave and points the next row back to allocator-row selection.

No MIR JSON shape, Rust metadata struct, verifier behavior, backend lowering,
runtime behavior, allocator behavior, packed backend activation, seed deletion,
or provider activation changed.

## Required Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
# [mir-metadata-catalog] ok module_keys=14 seed_keys=11

bash tools/checks/current_state_pointer_guard.sh
# [current-state-pointer-guard] ok

tools/checks/dev_gate.sh quick
# [dev-gate] profile=quick ok
```

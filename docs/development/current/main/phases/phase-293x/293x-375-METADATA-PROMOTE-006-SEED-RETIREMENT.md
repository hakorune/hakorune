# 293x-375 METADATA-PROMOTE-006 Seed Retirement

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-006` records and guards the retirement ledger for exact seed
routes.

This is a BoxShape docs / guard row only. It does not change MIR JSON shape,
Rust metadata structs, verifier behavior, backend lowering, runtime behavior,
or existing exact seed selection order.

## Responsibility

Canonical wording lives in:

```text
docs/reference/mir/metadata-facts-ssot.md
```

Guard owner:

```text
tools/checks/mir_metadata_catalog_guard.sh
```

## Guarded Contract

- Seed rows remain `ExperimentalSeedRoutes`, not CorePlan promotion targets.
- `exact_seed_backend_route` is only a selector over already-proven source seed
  payloads; it does not own legality.
- Each seed row now has an owner family, generic replacement target, and retire
  condition.
- New seed rows must update the ledger before they can be accepted.

## Stop Lines

- Do not promote seed payloads to CorePlan ownership.
- Do not make backend shims infer seed legality from raw helper names, method
  names, or app-specific block counts.
- Do not delete a seed row until its generic route replacement carries the
  same proof, selected value identity, publication boundary, and backend demand.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

---
Status: Landed
Date: 2026-04-24
Scope: Add the first MIR-side CoreMethodOp carrier slice for generic method routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-134-core-method-contract-inc-no-growth-guard-card.md
  - src/mir/core_method_op.rs
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-135 CoreMethodOp Carrier Card

## Goal

Land HCM-4: introduce a narrow MIR-visible CoreMethodOp carrier before moving
`.inc` consumers.

The first slice is intentionally one-family only:

```text
MapBox.has -> CoreMethodOp::MapHas
```

This does not change lowering behavior. Existing `generic_method_routes`
remain the backend compatibility path.

## Implementation

- Added `src/mir/core_method_op.rs`.
- Added `CoreMethodOp`, `CoreMethodOpProof`, `CoreMethodLoweringTier`, and
  `CoreMethodOpCarrier`.
- Attached `core_method: Option<CoreMethodOpCarrier>` to
  `GenericMethodRoute`.
- Populated the carrier only for the manifest-backed `MapBox.has` route.
- Emitted the carrier as `metadata.generic_method_routes[*].core_method` in
  MIR JSON.

Compatibility rows remain explicit:

```text
ArrayBox.has -> no CoreMethodOp carrier yet
RuntimeDataBox.has -> no CoreMethodOp carrier yet
```

## Boundary

- `.inc` still consumes `generic_method_routes` as before.
- No new method surface is accepted.
- No hot inline lowering is added.
- The carrier vocabulary is checked against
  `core_method_contract_manifest.json` in a focused Rust test.

## Proof

```bash
cargo fmt
cargo test -q core_method
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-5: convert one `.inc` consumer slice to prefer the CoreMethodOp carrier
  while keeping compatibility fallback unchanged.

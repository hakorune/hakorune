# 1805 - GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001

## Token

```text
GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001
```

## Purpose

Stop historical Source Selfhost row guards from pinning live
`CURRENT_STATE.latest_card` allowlists.

Historical row guards should validate their own card / fixture / non-claim
contract. The live current pointer is owned by:

```text
tools/checks/current_state_pointer_guard.sh
```

This reduces the recurring cost where every new row requires editing old row
guards only to extend a latest-card allowlist.

## Changed Boundary

```text
before:
  1801 / 1802 row guards required live latest_card to be one of a growing
  allowlist.

after:
  1801 / 1802 row guards require:
    latest_card present
    latest_card_path present
    latest_card_path references latest_card
    current_blocker_token remains the Source Selfhost design stop
```

## Acceptance

```text
current_state_pointer_guard = green
rust_lifecycle_source_selfhost_wider_route_selection_basis_guard = green
rust_lifecycle_source_selfhost_wider_route_selection_resolution_guard = green
rust_lifecycle_source_selfhost_docs_guard_maintenance_reduction_guard = green

current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Non-Claims

```text
no route repair
no family adoption decision
no wider route selection
no Source Selfhost claim
no weakening of historical row fixture checks
```

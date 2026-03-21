# phase29cc_wsm / p8

WSM-P8 bridge-retire readiness pin.

This family keeps the compat bridge retire execution path accepted-but-blocked
while `default-only` routing remains the live contract.

## Contains

- `phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh`

## Shared Helper

- `../../apps/phase29cc_wsm_cargo_test_common.sh`

## Contract

- The lock doc is the SSOT:
  - `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`
- The guard script must keep this pin reachable:
  - `tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh`

## Notes

- Keep new `phase29cc_wsm` work under this family tree.
- Do not add new `phase29cc_wsm_*` scripts back into `integration/apps/`.

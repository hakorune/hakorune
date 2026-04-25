---
Status: Landed
Date: 2026-04-26
Scope: Reconcile post-BuildBox worker inventory and thin the residual MIR-call MapBox.has fallback seam without changing fallback semantics.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-272-mir-call-maphas-surface-fallback-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-275-remaining-cleanup-inventory-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-286 MIR-Call MapHas Residual Seam Cleanup Card

## Goal

Continue compiler-clean cleanup after the BuildBox series without reopening the
blocked two-row `MapBox.has` fallback baseline.

This card is BoxShape-only:

- no new CoreMethod op
- no new lowering route
- no hot path change
- no MIR JSON fixture semantic change

## Worker Inventory Reconciliation

Worker inventory reported older blockers around direct Set boundary fixtures.
Those are already closed in current HEAD:

- `s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh` carries
  `generic_method.set` with `core_method.op=ArraySet`
- `s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh` carries
  `generic_method.set` with `core_method.op=MapSet`

Do not reopen Set route-policy pruning from that stale inventory without fresh
guard evidence.

## Current Residual Baseline

The no-growth guard remains intentionally pinned at:

```text
classifiers=2
rows=2
```

The remaining rows are the paired direct MIR-call fallback:

```text
classify_mir_call_receiver_surface box MapBox
classify_mir_call_method_surface method has
```

They are still owned by `291x-272` and remain blocked until direct
metadata-absent `MapBox.has` is retired, given an explicit non-surface
contract, or proven unreachable.

## Cleanup

The residual fallback path no longer needs receiver-family classification.
Only the exact surface pair matters:

```text
receiver surface = MapBox
method surface   = has
```

This card removes the dead `recv_family` seam from the MIR-call route policy
and updates the allowlist deletion condition so it points at the actual
`MapBox.has` sentinel instead of an older ArrayHas / RuntimeData condition.

## Boundary

- Keep `MapBox` / `has` string classifier rows.
- Keep the metadata-first route path ahead of the fallback.
- Keep the metadata-absent direct `MapBox.has` boundary smoke valid.
- Do not add new method-name or box-name classifier rows.

## Next

The next real reduction requires an owner-path change, not another blind prune:

1. retire metadata-absent direct `MapBox.has` as supported input, or
2. replace it with explicit metadata-bearing / non-surface contract, or
3. prove the fallback cannot be reached by supported producers.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

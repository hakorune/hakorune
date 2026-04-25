---
Status: Active
Date: 2026-04-25
Scope: Review `mir_call_route_policy` MapBox receiver-surface pruning after the ArrayBox receiver row was removed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-256-mir-call-arraybox-receiver-surface-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-257 MIR-Call MapBox Receiver-Surface Review Card

## Goal

Decide whether the remaining `MapBox` receiver-surface classifier in
`mir_call_route_policy` can be pruned after `ArrayBox` was removed.

## Probe

Temporary prune attempt:

- remove `HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX`
- remove `classify_mir_call_receiver_surface(... "MapBox" ...)`
- remove the `MapBox + has -> RUNTIME_MAP_HAS` route arm
- remove the matching `MapBox + has -> NEED_MAP_HAS` need-policy arm
- remove the no-growth allowlist row

Existing metadata-carrying smokes still passed, but that was not sufficient
because those smokes do not prove metadata-absent direct `MapBox.has`.

An exact metadata-absent direct boundary was then tested with a temporary MIR
JSON containing:

```text
MapBox.set metadata present
MapBox.has metadata absent
NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc
ny-llvmc --in /tmp/mapbox_has_no_metadata.*.json --out /tmp/*.o
```

Result:

```text
stage=mir_call_method ... bname=MapBox mname=has ... flags ... map_has:0
lane=none reason=unsupported_pure_shape
Error: unsupported pure shape for current backend recipe
```

## Decision

No safe prune.

`MapBox` receiver-surface classification is still required for
metadata-absent direct `MapBox.has` boundaries. The route-policy fallback and
need-policy declaration must stay paired:

```text
MapBox + has -> RUNTIME_MAP_HAS
MapBox + has -> NEED_MAP_HAS
```

## Boundary

- Keep `classify_mir_call_receiver_surface(... "MapBox" ...)`.
- Keep `HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX`.
- Keep the no-growth allowlist row.
- Do not add a workaround or silent fallback.
- Do not mix this with RuntimeDataBox receiver-surface review.

## Next Work

Proceed to the next task-order item:

```text
mir_call_receiver_surface RuntimeDataBox keep/prune review
```

RuntimeDataBox must be judged independently because it also carries
RuntimeData set/has/String compatibility fallback.

## Acceptance

```bash
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

---
Status: Landed
Date: 2026-04-24
Scope: Pin the landed MapLookupSameKey const-fold lowering and shared metadata-reader boundary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-147-mapget-maphas-fusion-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-148-mapget-maphas-fusion-has-const-probe-card.md
  - docs/development/current/main/phases/phase-291x/291x-149-maplookup-get-const-fold-card.md
  - tools/checks/map_lookup_fusion_reader_boundary_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 291x-150 MapLookup Fusion Guard Card

## Goal

Add small regression guards for the landed MapLookupSameKey const folds and the
shared metadata-reader seam:

```text
MapLookupSameKey.get_result -> stored_value_const
MapLookupSameKey.has_result -> constant 1
map_lookup_fusion_routes readers -> hako_llvmc_ffi_map_lookup_fusion_metadata.inc
```

H149 used perf and asm evidence to accept the keeper. This card turns the
structural part into a cheap daily contract so the route cannot silently drift
back to runtime helper calls or duplicated `.inc` metadata readers.

## Design

The boundary guard checks:

```text
only hako_llvmc_ffi_map_lookup_fusion_metadata.inc reads map_lookup_fusion_routes
get/has policy consumers call match_map_lookup_fusion_route_metadata(...)
```

The smoke uses the measured exact-front benchmark:

```text
benchmarks/bench_kilo_leaf_map_getset_has.hako
```

It checks:

```text
MIR JSON contains map_lookup.same_key with stored_value_const
entry LLVM IR does not call nyash.runtime_data.get_hh
entry LLVM IR does not call nyash.map.has_h
entry LLVM IR does not call nyash.map.probe_hi
```

The smoke does not judge perf counters. Perf remains owner-first evidence for
new optimization cards, not a quick-gate requirement.

## Boundary

- Do not add a new lowering.
- Do not add native i64-key storage.
- Do not add per-method `map_lookup_fusion_routes` readers outside the shared
  reader seam.
- Do not require `nyash.map.slot_load_hh` to disappear; H149 documents the
  remaining post-loop read boundary.
- Do not parse source shape in `.inc`; this is a smoke over emitted MIR/IR.

## Implementation

- Added `tools/checks/map_lookup_fusion_reader_boundary_guard.sh`.
- Added
  `tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh`.
- Wired both checks into `tools/checks/dev_gate.sh quick`.
- Added the new guard to `docs/tools/check-scripts-index.md`.

## Acceptance

- `bash tools/checks/map_lookup_fusion_reader_boundary_guard.sh`
- `env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh`
- `bash tools/checks/inc_codegen_thin_shim_guard.sh`
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `bash tools/checks/dev_gate.sh quick`

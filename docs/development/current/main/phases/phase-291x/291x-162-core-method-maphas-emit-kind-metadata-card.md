---
Status: Landed
Date: 2026-04-24
Scope: Make generic-method `has` emit-kind selection prefer MIR CoreMethod metadata before legacy method-name fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-161-core-method-route-policy-mirror-preflight-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc
---

# 291x-162 CoreMethod MapHas Emit-Kind Metadata Card

## Goal

Move one CoreMethod family from method-name-first dispatch to metadata-first
dispatch:

```text
generic_method_routes[].core_method.op = MapHas
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS
  -> legacy mname == "has" classifier remains fallback only
```

This is a one-family migration card. It does not delete allowlist rows and does
not change `has` lowering or helper symbols.

## Boundary

- Do not add hot inline lowering.
- Do not change `nyash.map.probe_hh`, `nyash.map.probe_hi`, or
  `nyash.runtime_data.has_hh` selection.
- Do not remove the legacy `mname == "has"` fallback in this card.
- Do not make `.inc` infer receiver legality; only consume the MIR-owned
  `core_method` carrier.

## Implementation

- Add a narrow dispatch helper that reads `generic_method_routes` at
  `(block, instruction_index)` and returns `EMIT_HAS` only for valid
  `core_method.op = MapHas` plus manifest proof and warm-direct-ABI tier.
- Thread the selected emit kind into `classify_generic_method_emit_plan(...)`
  so the plan does not re-run method-name classification after dispatch.
- Keep legacy classification as fallback when metadata is absent.

## Result

- `emit_mir_call_dispatch(...)` now tries the MapHas CoreMethod metadata
  selector before the legacy method-name classifier.
- `classify_generic_method_emit_plan(...)` accepts the preselected emit kind,
  so the metadata-first decision is not overwritten by a second classifier
  call.
- The `mname == "has"` allowlist row remains because legacy fallback is still
  live for metadata-absent routes.
- `has` helper symbols and lowering behavior are unchanged.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_routes
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```

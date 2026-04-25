# lang/src/runtime/meta — Compiler Semantic Tables

Scope:
- Own compiler-side semantic tables for stage2 cutover.
- Keep runtime kernel behavior in `runtime/kernel/`.
- Keep host transport in `runtime/host/`.

Responsibilities:
- `mir_call` route policy vocabularies.
- `mir_call` prepass need-flag tables.
- `mir_call` constructor/global/string-extern accept surfaces.
- `CoreMethodContract` seed rows for Array/String/Map method surfaces.

Non-goals:
- No kernel behavior.
- No host transport.
- No raw substrate / allocator backend.
- No direct LLVM emission.

Rule:
- This layer owns tables and policy words only.
- Native seams remain responsible for lowering, probing, and final code emission.

## Current modules

- `mir_call_route_policy_box.hako`
  - `MirCallRoutePolicy.classify_generic_method_route(...)`
  - registered transitional reference table only as of `291x-289`.
  - no active `.hako` or Rust caller owns route selection through this box.
  - do not add new route behavior here unless it is first wired to a generated
    CoreMethod/manifest contract.
- `mir_call_need_policy_box.hako`
  - `MirCallNeedPolicy.classify_need_flags(...)`
  - owns `mir_call` prepass need-flag tables.
- `mir_call_surface_policy_box.hako`
  - `MirCallSurfacePolicy.accept_surface(...)`
  - owns constructor/global/string-extern accept surfaces.
- `core_method_contract_box.hako`
  - `CoreMethodContractBox.rows()`
  - `CoreMethodContractBox.schema_fields()`
  - `CoreMethodContractBox.find(box_name, method_name, arity)`
  - `CoreMethodContractBox.core_op_for(box_name, method_name, arity)`
  - owns the first Array/String/Map compiler contract seed rows.
  - carries `lowering_tier` as contract data only.
  - does not emit `.inc` code or decide hot inline lowering.
- `generated/core_method_contract_manifest.json`
  - derived from `CoreMethodContractBox`
  - guarded by `tools/checks/core_method_contract_manifest_guard.sh`
  - paired with `tools/checks/core_method_contract_inc_no_growth_guard.sh`
    to stop new `.inc` method/box-name classifier growth
  - not a semantic owner; regenerate instead of hand-editing.

# lang/src/runtime/meta — Compiler Semantic Tables

Scope:
- Own compiler-side semantic tables for stage2 cutover.
- Keep runtime kernel behavior in `runtime/kernel/`.
- Keep host transport in `runtime/host/`.

Responsibilities:
- `mir_call` route-policy retirement notes and generated metadata handoff.
- `mir_call` prepass need-flag retirement notes and generated metadata handoff.
- `mir_call` constructor/global/string-extern surface-policy retirement notes.
- `CoreMethodContract` seed rows for Array/String/Map method surfaces.
- legacy/support meta utilities pending separate owner audit.

Non-goals:
- No kernel behavior.
- No host transport.
- No raw substrate / allocator backend.
- No direct LLVM emission.

Rule:
- This layer owns tables and policy words only.
- Native seams remain responsible for lowering, probing, and final code emission.

## Current modules

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

## Support exports pending audit

- `using_resolver.hako`
  - `UsingResolver.resolve(token)`
  - minimal meta support stub.
  - no external `selfhost.meta.UsingResolver` user was found in the
    `291x-295` inventory outside `UsingDecision`.
  - audit before moving or deleting; do not confuse it with the real
    Stage1/Pipeline using resolver boxes.
- `using_decision.hako`
  - `UsingDecision.decide(token)`
  - thin wrapper over `selfhost.meta.UsingResolver`.
  - audit together with `using_resolver.hako`.
- `json_shape_parser.hako`
  - `JsonShapeToMap.parse(json)` and helper functions.
  - support / JoinIR fixture utility; `JsonShapeToMap._read_value_from_pair/1`
    is referenced by JoinIR bridge dispatch tests.
  - keep separate from UsingResolver cleanup.

## Retired Modules

- `mir_call_route_policy_box.hako`
  - retired by `291x-290`.
  - had no active `.hako` or Rust caller after the CoreMethodContract route
    metadata migration.
  - route selection stays metadata-first through
    `lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc`.
  - do not reintroduce a by-name route table unless it is generated from
    CoreMethod/manifest metadata and wired as the actual producer.
- `mir_call_need_policy_box.hako`
  - owner-audited by `291x-291` and retired by `291x-292`.
  - had no active `.hako` or Rust caller; it is registered transitional
    vocabulary debt, not the executable need-policy owner.
  - need flags stay native through
    `lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc` until a
    generated producer or typed LoweringPlan owns those flags.
  - do not reintroduce a by-name need table unless it is generated from
    CoreMethod/manifest metadata and wired as the actual producer.
- `mir_call_surface_policy_box.hako`
  - owner-audited by `291x-293` and retired by `291x-294`.
  - had no active `.hako` or Rust caller; it is registered transitional
    vocabulary debt, not the executable surface-policy owner.
  - constructor/global/string-extern surfaces stay native through
    `lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc` until a
    generated producer or typed LoweringPlan owns those flags.
  - do not reintroduce a by-name surface table unless it is generated from
    CoreMethod/manifest metadata and wired as the actual producer.

# P381CJ LoweringPlan Metadata And Emitter Name SSOT

Date: 2026-05-05
Scope: sync long-lived SSOT docs after P381CG/P381CH/P381CI and lock the module-generic emitter naming boundary.

## Context

The implementation cards moved three global-call facts into Rust LoweringPlan
metadata:

- P381CG: `result_origin`
- P381CH: `definition_owner` and `emit_trace_consumer`
- P381CI: retired unused C direct-view predicates after those facts became
  MIR-owned

Those cards were current-lane evidence, but the durable JSON contract and
Stage0 inventory still needed the same facts so future cleanup does not
reconstruct them in C.

## LoweringPlan JSON Update

`docs/development/current/main/design/lowering-plan-json-v0-ssot.md` now lists
the three global-call metadata fields in the flat v0 entry shape and fixes the
consumer rule:

- `result_origin` is the source of Stage0 origin propagation
- `definition_owner` is the source of same-module definition-set selection
- `emit_trace_consumer` is the source of global-call route trace identity

C shims must not reconstruct those fields from `proof`, `route_proof`,
`target_shape`, or raw callee names.

## Historical Name Boundary

The Stage0 C files named
`hako_llvmc_ffi_module_generic_string_*` are historical names. Their active
responsibility is the module-generic same-module MIR function emitter for the
uniform ABI (`i64` handle/scalar) subset.

Rules:

- new docs and implementation should say "module-generic" or "uniform MIR
  function" for the responsibility
- the file stem must not be treated as permission to add string-only policy
- source-owner semantics still belong in Rust route facts or source-owner
  cleanup, not in new C by-name branches

## Body Emitter Inventory

Current line surface before the next implementation slice:

| file | lines | role |
| --- | ---: | --- |
| `hako_llvmc_ffi_module_generic_string_function_emit.inc` | 1713 | active module-generic function prepass/body emitter |
| `hako_llvmc_ffi_module_generic_string_method_views.inc` | 696 | direct generic-method LoweringPlan view predicates |
| `hako_llvmc_ffi_module_generic_string_plan.inc` | 138 | transitive same-module definition-set planner |
| `hako_llvmc_ffi_mir_call_need_policy.inc` | 733 | call-side need/policy helpers outside this slice |

Near-term cleanup candidates:

- replace remaining active direct global-call ownership checks with
  `definition_owner`
- keep void-sentinel no-dst handling separate until it becomes a general
  return-contract policy
- audit `method_views.inc` for repeated route/proof/shape boilerplate before
  touching behavior

## Verification

Commands:

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The durable docs now point future work at the MIR-owned metadata fields and
prevent the historical `module_generic_string` file names from becoming a
source of new string-only C policy.

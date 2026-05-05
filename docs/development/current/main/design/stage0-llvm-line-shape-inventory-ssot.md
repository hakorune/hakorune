---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: Stage0 LLVM line size guard, `GlobalCallTargetShape` inventory, and multi-function emitter boundary.
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/model.rs
---

# Stage0 LLVM Line Shape Inventory SSOT

## One-Line Rule

Stage0 does not know the selfhost compiler.

Stage0 knows Canonical MIR, uniform ABI, explicit route facts, runtime helper
routes, and fail-fast diagnostics.

If a change teaches Stage0 what a concrete `.hako` compiler helper means, stop
and choose one of these instead:

- source-owner cleanup
- MIR-owned fact / LoweringPlan contract
- generic MIR op support
- uniform multi-function MIR emitter

## Allowed Stage0 Knowledge

- Canonical MIR(JSON) input
- `i64`/handle-shaped uniform ABI at function boundaries
- explicit `LoweringPlan` / route facts
- runtime helper calls selected by explicit route facts
- module/function declarations and calls by MIR symbol
- fail-fast diagnostics for unsupported routes

## Disallowed Stage0 Knowledge

- parser policy
- MirBuilder policy
- JsonFrag / normalizer policy
- BuildBox / ParserBox / FuncLoweringBox semantic policy
- source helper body clones
- new body-specific C shim emitters for one `.hako` owner

## GlobalCallTargetShape Status

`GlobalCallTargetShape` variants are not all equal. A new variant is allowed
only when the card updates this table and states why uniform MIR function
emission cannot handle the blocker yet.

| Shape | Status | Owner Reading | Removal Path |
| --- | --- | --- | --- |
| `Unknown` | fail-fast | no accepted target shape | none; keep as unsupported diagnostic |
| `NumericI64Leaf` | permanent ABI primitive | leaf scalar function body for first same-module call emission | keep as scalar ABI bootstrap unless uniform emitter makes the leaf special-case redundant |
| `GenericI64Body` | permanent candidate | scalar/bool/i64 ABI helper classifier | keep narrow to scalar facts; do not add collection or compiler-owner semantics |
| `GenericPureStringBody` | permanent candidate with shrink target | string-handle ABI helper classifier | keep only string flow; move collection/normalizer/source-owner meaning out |

Void/logging direct calls are no longer a `GlobalCallTargetShape` variant. MIR
still records them as direct ABI targets with
`proof=typed_global_call_generic_string_void_logging` and
`return_shape=void_sentinel_i64_zero`; `target_shape` is omitted because the
proof/return contract is now the SSOT for this retired capsule.

Static string array direct calls are no longer a `GlobalCallTargetShape` variant.
MIR still records them as direct ABI targets with
`proof=typed_global_call_static_string_array` and `return_shape=array_handle`;
`target_shape` is omitted because the proof/return contract plus
`result_origin=array_string_birth` metadata are now the SSOT for this retired
origin-carrying capsule. Its body is emitted as an ordinary module generic MIR
function; array append sites inside the body are accepted only through
`generic_method.push` route facts.

MIR schema map constructor direct calls are no longer a `GlobalCallTargetShape`
variant. MIR still records them as direct ABI targets with
`proof=typed_global_call_mir_schema_map_constructor` and
`return_shape=map_handle`; `target_shape` is omitted because the proof/return
contract plus `result_origin=map_birth` metadata are now the SSOT for this
retired origin-carrying capsule.

Parser Program(JSON) direct calls are no longer a `GlobalCallTargetShape`
variant. MIR still records them as direct ABI targets with
`proof=typed_global_call_parser_program_json` and
`return_shape=string_handle`; `target_shape` is omitted because the proof/return
contract plus `result_origin=string` metadata are now the SSOT for this retired
source-execution capsule. Its Stage0 body definition is now emitted by the
module generic MIR function emitter; the old parser-only body clone has been
removed.

BoxTypeInspector describe direct calls are no longer a `GlobalCallTargetShape`
variant. MIR still records them as direct ABI targets with
`proof=typed_global_call_box_type_inspector_describe` and
`return_shape=map_handle`; `target_shape` is omitted because the proof/return
contract plus `result_origin=map_birth` metadata are now the SSOT for this
retired source-owner capsule. The active source-owner consumers already use
scalar predicates (`is_map` / `is_array`); Stage0 now plans, emits, traces, and
propagates the map origin through LoweringPlan metadata views instead of a
describe-specific call-site branch.

PatternUtil local-value probe direct calls are no longer a
`GlobalCallTargetShape` variant. MIR still records them as direct ABI targets
with `proof=typed_global_call_pattern_util_local_value_probe` and
`return_shape=mixed_runtime_i64_or_handle`; `target_shape` is omitted because
the proof/return contract is now the SSOT for this retired mixed
scalar/handle capsule. Recursive child-probe recognition also consumes the
proof/return contract instead of the legacy shape string. Stage0 plans and
emits the backend body through the shared module-generic LoweringPlan helpers
instead of a pattern-probe-specific call-site branch.

Generic string-or-void sentinel direct calls are no longer a
`GlobalCallTargetShape` variant. MIR still records them as direct ABI targets
with `proof=typed_global_call_generic_string_or_void_sentinel` and
`return_shape=string_handle_or_null`; `target_shape` is omitted because the
proof/return contract plus `result_origin=string` metadata are now the SSOT for
this retired sentinel capsule. Generic-method string-origin consumers also read
the proof/return facts instead of the legacy shape string.

## Missing Multi-Function Emitter Policy

`missing_multi_function_emitter` is not permission to add another body shape.

Default fix order:

1. Try source-owner cleanup that removes temporary collection/sentinel
   plumbing.
2. Consume existing MIR-owned facts or add a narrow fact at the callsite.
3. Add generic MIR op support if the missing piece is an opcode/ABI gap.
4. Add the uniform multi-function MIR emitter.

The desired Stage0 emitter shape is:

```text
MIR function A
  calls MIR function B
  calls MIR function C

ABI:
  params = i64 handles/scalars
  return = i64 handle/scalar
```

The backend must emit calls by MIR symbol/route fact. It must not rediscover
callee meaning from raw `.hako` owner names or from C-side capsule proof-name
lists. Global-call result origin, definition ownership, and trace consumer
selection are MIR-owned LoweringPlan metadata fields.

The current Stage0 C file names under
`hako_llvmc_ffi_module_generic_string_*` are historical names. Their active
responsibility is the module-generic same-module MIR function emitter for
uniform ABI (`i64` handle/scalar) functions. New code and docs should describe
the responsibility as "module-generic" or "uniform MIR function" and must not
interpret the file stem as permission to add string-only or source-owner
semantics.

## Card Checklist

Every source-execution cleanup card must answer:

- What does this change reduce?
- Does it add a `GlobalCallTargetShape`?
- Does it add a C shim body-specific emitter?
- Does it teach Stage0 a source helper meaning?
- If it is a temporary capsule, what removes it?

If the answer adds Stage0 shape/C-shim/source-helper meaning, the card should
stop unless it has an explicit temporary-capsule row and a removal path here.

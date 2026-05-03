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
| `GenericStringOrVoidSentinelBody` | temporary capsule | string/void sentinel bridge for current source-execution blockers | replace by uniform MIR function emitter or source-owner sentinel cleanup |
| `GenericStringVoidLoggingBody` | temporary capsule | void/logging scalar bridge | replace by uniform MIR function emitter or source-owner logging contract cleanup |
| `ParserProgramJsonBody` | temporary capsule | parser Program(JSON v0) route validator | replace by uniform MIR function emitter or parser/source owner cleanup |
| `ProgramJsonEmitBody` | temporary capsule | Program(JSON v0) emit wrapper validator | replace by uniform MIR function emitter or owner-local direct MIR route |
| `JsonFragInstructionArrayNormalizerBody` | temporary capsule | JsonFrag instruction-array normalizer validator | replace by uniform MIR function emitter; do not widen generic string/collection bodies |
| `StaticStringArrayBody` | temporary capsule | static array construction bridge | replace by uniform MIR function emitter or explicit runtime helper route |
| `BuilderRegistryDispatchBody` | temporary capsule | builder registry dispatch bridge | replace by uniform MIR function emitter or source-owner dispatch cleanup |
| `MirSchemaMapConstructorBody` | temporary capsule | MIR schema map constructor bridge | replace by uniform MIR function emitter or MIR-owned schema facts |
| `BoxTypeInspectorDescribeBody` | temporary capsule | box-type inspector map-return bridge | replace by source-owner scalar predicates or uniform MIR function emitter |
| `PatternUtilLocalValueProbeBody` | temporary capsule | pattern util local-value probe bridge | replace by source-owner text/scalar cleanup or uniform MIR function emitter |

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
callee meaning from raw `.hako` owner names.

## Card Checklist

Every source-execution cleanup card must answer:

- What does this change reduce?
- Does it add a `GlobalCallTargetShape`?
- Does it add a C shim body-specific emitter?
- Does it teach Stage0 a source helper meaning?
- If it is a temporary capsule, what removes it?

If the answer adds Stage0 shape/C-shim/source-helper meaning, the card should
stop unless it has an explicit temporary-capsule row and a removal path here.

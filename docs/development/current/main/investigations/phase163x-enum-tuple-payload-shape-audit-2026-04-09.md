---
Status: Active Investigation
Date: 2026-04-09
Scope: tuple enum payload shape audit across AST / Stage1 / Program JSON v0 / JSON v0 bridge / MIR.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - src/ast/mod.rs
  - src/parser/declarations/enum_def/mod.rs
  - src/parser/expr/match_expr.rs
  - src/stage1/program_json_v0/authority.rs
  - src/stage1/program_json_v0/lowering.rs
  - src/stage1/program_json_v0/record_payload.rs
  - src/runner/json_v0_bridge/ast.rs
  - src/runner/json_v0_bridge/lowering/expr/sum_ops.rs
  - src/mir/function.rs
  - src/mir/instruction.rs
---

# Phase 163x Enum Tuple Payload Shape Audit

## Findings

- Parser / AST currently model enum variants as:
  - unit variants
  - single-payload tuple variants through `payload_type_name: Option<String>`
  - record variants through `record_field_decls: Vec<FieldDecl>`
- tuple multi-payload is rejected at the declaration surface today.
  - `src/parser/declarations/enum_def/mod.rs`
  - `,` inside `Variant(T, U, ...)` currently fails with `single payload variant in the current enum surface`
- known-enum match shorthand is also single-payload on the tuple lane today.
  - `src/parser/expr/match_expr.rs`
  - `Variant(x)` is supported
  - `Variant(x, y)` has no current AST carrier; record patterns use a hidden payload binding plus `BlockExpr` prelude bindings instead
- Stage1 Program JSON v0 keeps the canonical enum inventory singular on the sum lane.
  - `src/stage1/program_json_v0/authority.rs`
  - enum inventory emits `variants[].payload_type`
  - record variants already boxify to `__NyEnumPayload_<Enum>_<Variant>` and reuse that single `payload_type`
- Stage1 expression lowering follows the same 0/1 payload assumption.
  - `src/stage1/program_json_v0/lowering.rs`
  - constructor arity is `0`, `1`, or record-field count
  - record constructors and record match arms lower through one hidden payload box value
- Program JSON v0 schema is singular on the enum lane.
  - `src/runner/json_v0_bridge/ast.rs`
  - `EnumVariantDeclV0.payload_type`
  - `EnumMatchArmV0.bind`
  - `ExprV0::EnumCtor.args` is an array, but the enum inventory still describes only one payload slot
- JSON v0 bridge lowering is explicit about the same contract.
  - `src/runner/json_v0_bridge/lowering/expr/sum_ops.rs`
  - resolved arity is `usize::from(payload_type_name.is_some())`
  - `SumMake` receives `payload: Option<ValueId>`
  - `SumProject` projects exactly one payload value when a bind exists
- MIR metadata and canonical sum instructions are also singular today.
  - `src/mir/function.rs`
  - `MirEnumVariantDecl.payload_type_name: Option<String>`
  - `src/mir/instruction.rs`
  - `SumMake.payload: Option<ValueId>`
  - `SumProject` has one projected payload result

## Test Coverage After This Audit

- parser surface rejection is pinned in `src/tests/parser_enum_surface.rs`
- Stage1 strict source-route rejection is pinned in `src/stage1/program_json_v0.rs`
- JSON v0 bridge single-payload ctor contract is pinned in `src/runner/json_v0_bridge/mod.rs`

## Recommendation

- keep the canonical sum lane single-payload for the current wave
- if tuple multi-payload work resumes, lower it through the existing synthetic hidden payload box route first
- do not widen `EnumCtor` / `EnumMatch` / `SumMake` / `SumProject` in the same cut unless a separate canonical-sum design decision lands first

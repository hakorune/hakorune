---
Status: Active Investigation
Date: 2026-04-09
Scope: aggregate-truth audit across AST / Stage1 / Program JSON v0 / JSON v0 bridge / MIR metadata under the lifecycle-value parent SSOT.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/development/current/main/investigations/phase163x-enum-tuple-payload-shape-audit-2026-04-09.md
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
  - src/runner/mir_json_emit/mod.rs
---

# Phase 163x Aggregate Truth Audit

## Findings

### 1. AST / parser still encode enum payload truth as `0|1 payload slot + record sidecar`

- `src/ast/mod.rs`
  - `EnumVariantDecl` keeps:
    - `payload_type_name: Option<String>`
    - `record_field_decls: Vec<FieldDecl>`
  - current AST can describe:
    - unit variants
    - single-payload tuple variants
    - record variants
  - current AST cannot describe tuple multi-payload as aggregate truth.
- `src/parser/declarations/enum_def/mod.rs`
  - `Variant(T, U, ...)` is rejected with `single payload variant in the current enum surface`.
- `src/parser/expr/match_expr.rs`
  - tuple shorthand still binds through one `binding_name`
  - record shorthand is lowered through a hidden payload binding plus `BlockExpr` prelude bindings
  - there is no first-class tuple aggregate binder for `Variant(x, y, ...)`.

Classification:

- semantic blocker for tuple multi-payload / aggregate-first sum truth

### 2. Stage1 Program JSON v0 still converts record payload shape into hidden payload boxes

- `src/stage1/program_json_v0/record_payload.rs`
  - `enum_variant_payload_type_name()` maps record variants to synthetic hidden payload box names
    `__NyEnumPayload_<Enum>_<Variant>`.
- `src/stage1/program_json_v0/authority.rs`
  - enum inventory still publishes one `payload_type` slot per variant.
- `src/stage1/program_json_v0/lowering.rs`
  - record constructors lower through:
    - one `New` hidden payload box
    - then one `EnumCtor.args` payload slot
  - enum match arms still expose one `bind` and one `payload_type`.

Classification:

- compat fallback leaking into semantic lowering
- still a blocker for aggregate-first truth, even though it is acceptable as a temporary transport for JSON v0

### 3. JSON v0 bridge and canonical MIR stay singular on the sum lane

- `src/runner/json_v0_bridge/ast.rs`
  - `EnumVariantDeclV0.payload_type: Option<String>`
  - `EnumMatchArmV0.bind: Option<String>`
- `src/runner/json_v0_bridge/lowering/expr/sum_ops.rs`
  - ctor arity is `usize::from(payload_type_name.is_some())`
  - `multi-payload variants are outside MVP`
  - `SumMake` is fed one `payload: Option<ValueId>`
  - `SumProject` yields one projected payload value
- `src/mir/function.rs`
  - `MirEnumVariantDecl.payload_type_name: Option<String>`
- `src/mir/instruction.rs`
  - `SumMake.payload: Option<ValueId>`
  - `SumProject` projects one payload value
- `src/runner/mir_json_emit/mod.rs`
  - MIR JSON still re-emits enum inventory as one `payload_type` slot.

Classification:

- semantic blocker
- this is the canonical shape that keeps tuple multi-payload out of scope today

### 4. Record-field truth survives only through sidecar user-box metadata

- `src/stage1/program_json_v0/record_payload.rs`
  - record fields are preserved by synthetic payload box declarations
- `src/runner/json_v0_bridge/lowering.rs`
  - hidden payload boxes are copied into `module.metadata.user_box_decls` and
    `module.metadata.user_box_field_decls`
- `src/mir/function.rs`
  - user-box names and typed field declarations remain parallel sidecars

Classification:

- compat-only fallback
- acceptable as transport/debug inventory
- not acceptable as the long-term semantic truth for enum payload shape

## Scope Judgment

- tuple multi-payload is not a parser-only gap
- widening it now would require one coordinated shape change across:
  - AST
  - parser shorthand match
  - Stage1 Program JSON v0
  - JSON v0 bridge AST/lowering
  - MIR metadata
  - canonical `SumMake` / `SumProject`

## Decision Input

This audit says the next cut should **not** be tuple multi-payload first.

Reason:

1. the current aggregate gap is architectural, not local
2. record payloads already rely on compat-only hidden boxes
3. a tuple multi-payload push before thin-entry inventory would widen the current single-slot contract without first deciding where local values may remain unboxed

Recommended next keeper:

- thin-entry inventory first
- then revisit tuple multi-payload only after the semantic/compat boundary is explicit

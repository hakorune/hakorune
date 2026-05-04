---
Status: Active
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AQ, Option/null/no-match policy lock and implementation task map
Related:
  - docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md
  - docs/reference/language/option.md
  - docs/reference/language/types.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
---

# P381AQ: Option / Null / No-Match Task Map

## Problem

`Option<T>` has been discussed before, but without a durable SSOT it can be
accidentally deleted or conflated with compiler helper no-match. Recent P381
cleanup also showed that using `null` and numeric `0/1` as helper sentinels
creates direct LLVM route type pressure.

We need to preserve the public Option direction while keeping Stage0 small.

## Historical Reference

`Option` has existed in earlier design and implementation lines, but those
lines are historical references, not current canonical semantics:

- `e441b2ba2` / `4cc25f3fb` added a Box-first library implementation
  (`apps/lib/boxes/option.hako`, `apps/lib/boxes/result.hako`) plus
  Option/Result proposal docs. That implementation used `OptionBox` /
  `ResultBox` as library boxes.
- `6ed7ce368` documented an Optional/Null MVP direction with `get?` /
  `indexOf?` style APIs and an eventual OptionalBox/MaybeBox.
- Phase 12.7-era archived/reference language material records `?`
  propagation and `ResultBox`-style error handling as future/public surface
  direction. Treat that as historical context until the current enum Option /
  Result semantics are locked.
- `docs/reference/language/strings.md` still records that Option/Maybe may
  replace null-like APIs in a future revision.
- `7e0161bb3` pinned an internal CondProfile/String/Option ownership policy;
  that `Option<T>` is analysis state, not public language semantics.

Decision:

```text
Historical OptionBox / ResultBox:
  implementation reference only

Current canonical public optional value:
  enum Option<T> { None, Some(T) }
```

If an `OptionBox` facade is useful later for compatibility, it must be layered
over the enum Option semantics. It must not become the language meaning.

## Decision

Lock the boundary:

```text
null / void:
  language no-value literals

Option<T>:
  public null-free optional value
  enum Option<T> { None, Some(T) }

compiler helper no-match:
  owner-local text sentinel or tagged text carrier
```

Do not use `Option<T>` as the Stage0 helper no-match carrier.

Invariants:

- `Option::None` is not `null`
- `Option::Some(null)` is invalid
- `Option::Some(void)` is invalid
- `null` remains a source-level Void/none literal for dynamic and legacy APIs

## Task Order

### AQ-1. Docs policy lock

Add:

- `docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md`
- `docs/reference/language/option.md`

Update:

- `docs/reference/language/types.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/README.md`
- `docs/development/current/main/design/README.md`
- `CURRENT_STATE.toml`

Record:

- historical OptionBox / ResultBox references
- current enum Option decision
- Stage0 no-match rule

### AQ-2. Dual parser inventory

Before implementation, inventory both parser fronts:

- Rust parser enum/constructor/match support
- `.hako` parser enum/constructor/match support
- AST JSON / Program(JSON) fields
- Stage1 `EnumCtor` / `EnumMatch` route

Acceptance:

- one table lists Rust support, `.hako` support, and gaps
- no code changes

### AQ-2 inventory snapshot (2026-05-04)

| Surface | Rust parser front | `.hako` parser front | AST / Program(JSON) / Stage1 route | Gap to close |
| --- | --- | --- | --- | --- |
| `enum` declaration | Supported. `src/parser/declarations/enum_def/mod.rs` parses `enum Name<T> { ... }`, including unit, single-payload, tuple, and record variants. Coverage exists in `src/tests/parser_enum_surface.rs`. | Not supported. `lang/src/compiler/parser/parser_box.hako` + `stmt/parser_stmt_box/core.hako` only coordinate statement JSON parsing; there is no `enum` keyword route. | Rust AST carries `EnumDeclaration`; `src/macro/ast_json/roundtrip.rs` and `src/macro/ast_json/joinir_compat.rs` serialize it; `src/stage1/program_json_v0/tests.rs` shows `enum_decls` in Program(JSON v0). | Selfhost front cannot yet declare public `Option<T>` / other enums. |
| Plain constructor `new Box(args)` | Supported as `ASTNode::New` in `src/parser/expr/primary.rs`. | Supported. `lang/src/compiler/parser/expr/parser_expr_box.hako` emits `{"type":"New","class":...,"args":[...]}`. | `src/stage1/program_json_v0/lowering.rs` lowers `ASTNode::New` to `"type":"New"`; `src/runner/json_v0_bridge/lowering/expr.rs` lowers `ExprV0::New`. | No AQ-2 blocker here; constructor parity already exists for non-enum owners. |
| Enum variant constructor with payloads | Partially supported. `src/parser/expr/primary.rs` parses `Type::Variant(...)` and `Type::Variant { ... }` as `ASTNode::FromCall`; record fields are validated/reordered and Stage1 lowers them to `"type":"EnumCtor"` (`src/stage1/program_json_v0/lowering.rs`). Covered by `src/tests/parser_enum_match.rs` and `src/stage1/program_json_v0/tests.rs` for `Option::Some(1)`, `Token::Ident { ... }`, and `Pair::Both(1, 2)`. | Not supported. `lang/src/compiler/parser/expr/parser_expr_box.hako` has `new`, `Call`, `Method`, and `peek`, but no `::`-qualified constructor path and no `"EnumCtor"` emission. | Program(JSON v0) already has `"type":"EnumCtor"` and `ExprV0::EnumCtor`; `src/runner/json_v0_bridge/lowering/expr.rs` lowers it through `sum_ops`. | AQ-3 can reuse the Stage1/bridge route, but the selfhost parser must learn enum ctors. |
| Bare unit enum constructor `Option::None` | Not yet supported on the Rust front. `src/parser/expr/primary.rs` currently requires `(` or `{` after `Type::Variant`, so a bare `Type::Variant` form is rejected even though AQ-3 wants `Option::None`. There is also no existing `Option::None` parser test yet. | Not supported. Same missing `::` / enum-ctor surface as above. | Program(JSON v0) `EnumCtor` shape can already encode zero-arg constructors; the missing piece is parser acceptance on both fronts. | AQ-3 must add bare unit variant parsing on both fronts. |
| Literal/general `match` | Supported. `src/parser/expr/match_expr.rs` builds `MatchExpr` for literal-arm match and guarded fallback lowering. | Not supported. The selfhost expression parser has no `match` keyword path. | Rust AST JSON roundtrip supports `MatchExpr`; Program(JSON v0) has `"type":"Match"`. | Selfhost front still lacks any `match` surface. |
| Known-enum shorthand match | Supported on the Rust front. `src/parser/expr/match_expr.rs` + `match_expr_impl.rs` resolve `Some(v)`, `None`, record patterns, and tuple patterns into `EnumMatchExpr`; exhaustiveness is enforced, while guarded enum shorthand is still explicitly rejected in MVP. Coverage exists in `src/tests/parser_enum_match.rs`. | Not supported. The selfhost parser only has `peek ... { "label" => expr, else => expr }` via `lang/src/compiler/parser/expr/parser_peek_box.hako`; it does not recognize `match`, enum shorthand arms, or pattern heads. | Rust AST JSON roundtrip / joinir compat support `EnumMatchExpr`; Program(JSON v0) has `"type":"EnumMatch"` and `src/runner/json_v0_bridge/lowering/expr.rs` lowers it via var-scope-only sum-op routing. | AQ-3 needs selfhost `match` support; AQ-4 can then enforce null-free `Some(...)` on the shared enum lane. |
| AST JSON coverage for enum work | Partial. `EnumDeclaration` and `EnumMatchExpr` are present in `src/macro/ast_json/roundtrip.rs` and `joinir_compat.rs`. | N/A: the `.hako` parser does not produce Rust AST; it emits Program(JSON v0) directly. | No visible `FromCall` / enum-constructor serializer path was found in `src/macro/ast_json/*`, so enum ctor AST transport is weaker than enum decl / enum match transport today. | If AQ-3/AQ-4 need AST JSON transport for enum ctors, add or restore explicit `FromCall` support there. |

Notes:

- The main AQ-2 asymmetry is not Stage1 or bridge lowering; those already know
  `EnumCtor` / `EnumMatch`. The asymmetry is front-end acceptance.
- Rust front is ahead on enum surface, but still misses the public bare unit
  constructor form required by AQ-3 (`Option::None` without `()`).
- `.hako` parser is still a minimal Program(JSON v0) front: it has ordinary
  `new` constructors and `peek`, but no `enum`, `Type::Variant`, or `match`.

### AQ-3. Public Option owner restore/add

Implement or restore the public `Option<T>` owner on the existing enum surface.

Rules:

- no new syntax in this step
- no Stage0 no-match use
- no backend body-specific emitter

Acceptance:

- `Option::Some(1)` and `Option::None` parse on both fronts
- known-enum match works on both fronts

### AQ-3 progress snapshot (2026-05-04)

Result:

- Rust parser front now accepts bare unit enum ctors on the existing enum lane, so
  `Option::None` no longer requires `()`.
- Selfhost `.hako` parser front now restores the minimal public enum surface needed
  for AQ-3:
  - full-source enum inventory scan
  - `Type::Variant(...)` and bare unit `Type::Variant` emission as `EnumCtor`
  - known-enum shorthand `match` emission as `EnumMatch`
  - root Program(JSON v0) `enum_decls` injection for the existing bridge route

Validation:

- Rust parser coverage: `parse_unit_enum_ctor_without_parentheses_keeps_enum_ctor_shape`
- Stage1 Program(JSON v0) coverage: `source_to_program_json_v0_emits_unit_enum_ctor`
- Existing known-enum match coverage remains green on the Rust front
- Selfhost BuildBox coverage:
  `build_box_emit_program_json_v0_preserves_enum_ctor_and_enum_match_surface`
  proves the `.hako` parser emits `enum_decls`, unit/payload `EnumCtor`, and
  `EnumMatch` on the shared Program(JSON v0) lane

Next:

- AQ-4 null-free `Option::Some(...)` guard on the shared enum lane

### AQ-4. Null-free Some guard

Reject:

```hako
Option::Some(null)
Option::Some(void)
```

Acceptance:

- static reject when known at parse/analysis
- otherwise runtime fail-fast at construction
- positive non-null payloads continue to pass

### AQ-4 progress snapshot (2026-05-04)

Result:

- The shared enum lane now rejects nullish public `Option::Some(...)` payloads without
  adding new syntax or backend-specific emitters.
- Rust Stage1 / Program(JSON v0) lowering now rejects statically known
  `Option::Some(null)` and `Option::Some(void)`.
- Program(JSON v0) → MIR lowering now rejects direct JSON `Null` payloads too, so the
  selfhost `.hako` front inherits the same compile-time contract on the shared lane.
- VM `VariantMake` now fail-fast rejects runtime-nullish payloads for `Option::Some`,
  covering cases where compile-time analysis only sees a variable or other non-literal.

Validation:

- Rust Stage1 static reject:
  `source_to_program_json_v0_rejects_option_some_null_payload`
- Rust Stage1 static reject for `void`:
  `source_to_program_json_v0_rejects_option_some_void_payload`
- Shared Program(JSON v0) compile-time reject:
  `parse_json_v0_to_module_rejects_option_some_null_payload`
- Runtime fail-fast on non-literal nullish payload:
  `vm_fails_fast_when_option_some_payload_evaluates_to_nullish_value`
- Positive keep-pass:
  `vm_allows_option_some_non_null_payload`

Next:

- AQ-6 remains optional; AQ-1 through AQ-5 now land on the shared enum lane

### AQ-5. Optional sugar on the shared enum lane

Implemented:

- `some expr`
- `none`
- `if some v = expr { ... } else { ... }`

Result:

- Rust parser and selfhost `.hako` parser now both accept the public sugar surface
  while reusing the existing `Option` enum lane.
- `some expr` lowers to `Option::Some(expr)`.
- `none` lowers to `Option::None`.
- `if some v = expr { ... } else { ... }` rewrites to:
  - a hidden subject `Local`
  - an `If` whose condition is `EnumMatch(Option, ...) -> Bool`
  - a leading payload-binding `Local` inside the then-body
- No new Program(JSON v0) node shape or runtime representation was introduced.

Validation:

- Rust parser sugar ctor:
  `parse_option_sugar_some_becomes_option_ctor`
- Rust parser `if some` rewrite:
  `parse_if_some_sugar_rewrites_to_scopebox_over_enum_match_lane`
- Stage1 sugar ctor lowering:
  `source_to_program_json_v0_emits_option_sugar_some_and_none`
- Stage1 `if some` lowering:
  `source_to_program_json_v0_rewrites_if_some_sugar_to_local_plus_if`
- Selfhost BuildBox parity:
  `build_box_emit_program_json_v0_preserves_option_sugar_surface`

Deferred:

- `?` propagation

### AQ-6. Optional OptionBox compatibility facade

Only if needed after AQ-3/AQ-4, add a compatibility facade:

```text
OptionBox facade -> enum Option<T> semantics
```

Rules:

- facade is not language semantics
- facade does not carry Stage0 no-match
- facade docs must state the removal or compatibility purpose

## Non-Goals

- do not replace `null`
- do not use `Option` for Stage0 no-match
- do not widen Stage0 body shapes
- do not add C shim body-specific emitters
- do not add backend null/string or i1/i64 repair

## Result

AQ-1 through AQ-5 are now recorded in this card:

- AQ-2 locked the dual-parser inventory.
- AQ-3 restored dual-front enum ctor / match parity on the shared lane.
- AQ-4 made public `Option::Some(...)` null-free with compile-time and runtime guards.
- AQ-5 landed `some` / `none` / `if some` sugar without introducing a new IR or runtime path.

Next step:

- AQ-6 stays optional and should only land if a compatibility facade is still needed

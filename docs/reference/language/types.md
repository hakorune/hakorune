# Type System (SSOT)

Status: Draft / active (2025-12)
Decision: accepted (責務分離完了、挙動変更なし)
- Note: Decision: accepted は Phase 15 の責務分離（挙動不変）の受理。仕様全体は Status: Draft/active として継続更新する。
- 受理条件達成: phase29bq_fast_gate_vm.sh + phase29bp_planner_required_dev_gate_v4_vm.sh + phase29ae_regression_pack_vm.sh 全緑
- T2/T4/T5 完了: RuntimeTypeTag（入口分類）、RuntimeTypeSpec（意味論SSOT）、MirType完全遮断

This document defines the **current, executable** type semantics of Nyash/Hakorune.
Implementation is currently anchored to the Rust VM (`src/backend/mir_interpreter/*`).

If another backend differs from this document, treat it as a bug unless explicitly noted.

Coercion SSOT status:
- Decisions: `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`
- Implemented (VM + LLVM parity): Phase 275 P0

---

## 1. Big Picture

- Nyash is **dynamically typed**: runtime values carry a tag (Integer/Float/Bool/String/Void/BoxRef/Future).
- `local` declares a variable; **re-assignment is allowed** (single keyword policy).
- There is currently no static type checker. Some parts of MIR carry **type facts** as metadata for optimization / routing, not for semantics.

### First-class enum surface (current landing)

- `enum Name<T> { ... }` now parses as a first-class declaration surface.
- accepted constructor surface now includes:
  - `Type::Variant(...)`
  - narrow record constructors like `Type::Variant { name: expr }`
- Stage1 Program JSON now carries:
  - `enum_decls`
  - `EnumCtor`
  - synthetic hidden payload box declarations for narrow record variants
- known-enum shorthand `match` is now landed on the same narrow parser / AST / Stage1 lane:
  - shorthand patterns like `Some(v)` / `None`
  - narrow record shorthand like `Ident { name }`
  - Stage1 `EnumMatch`
  - exhaustiveness against the known enum inventory
  - exact field-set checking for record constructors / patterns
- canonical sum MIR lowering is now landed too:
  - `EnumCtor` lowers to `SumMake`
  - `EnumMatch` lowers to `SumTag` + compare/branch + `SumProject`
- VM/LLVM fallback runtime semantics are landed on the current route too:
  - enum values use the existing synthetic sum runtime box `__NySum_<Enum>` where fallback representation is needed
  - narrow record payloads use synthetic hidden payload boxes `__NyEnumPayload_<Enum>_<Variant>`
  - LLVM recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when local payload facts are known
- Current guardrails:
  - unknown/genuinely dynamic payloads still stay on boxed-handle fallback
  - record shorthand block bodies and multi-payload variants are still deferred

Terminology (SSOT):
- **Runtime type**: what the VM executes on (`VMValue`).
- **MIR type facts**: builder annotations (`MirType`, `value_types`, `value_origin_newbox`, `TypeCertainty`).

### Null vs Void (SSOT)

Nyash has two surface literals: `null` and `void`.

SSOT policy:
- `null` is the source-level “none” literal used in APIs like `toIntOrNull()` and optional returns.
- `void` is the “no value” literal (and is also the value produced by expressions/statements that do not yield a value).
- At runtime, both are represented as the same “no value” concept (`Void`). Treat `null` as a syntax-level alias of `void` unless a backend explicitly documents a difference (differences are bugs).

Practical consequence:
- `x == null` and `x == void` are equivalent checks.
- `WeakRef.weak_to_strong()` returns `null` on failure (i.e., `void` / none).

---

## 2. Variables and Re-assignment

- `local x` / `local x = expr` introduces a mutable local variable.
- `local x` is treated as `local x = null` (i.e., a `Void` value) unless an initializer is provided.
- Re-assignment is always allowed: `x = expr`.
- “Immutable locals” (let/const) are not part of the language today; they can be introduced later as lint/strict checks without changing core semantics.

Note: Field type annotations like `field: TypeBox` exist in syntax, but are currently **not enforced** as a type contract (docs-only) — see `LANGUAGE_REFERENCE_2025.md`.

---

## 3. Boolean Context (truthiness)

Boolean context means:
- `if (cond) { ... }`
- `loop(cond) { ... }`
- `!cond`
- branch conditions generated from `&&` / `||` lowering

Conditions accept any value; truthiness is applied. A Bool-only restriction is not part of the language.

Runtime rule (SSOT) is implemented by `to_bool_vm` (`src/backend/abi_util.rs`):

- `Bool` → itself
- `Integer` → `0` is false; non-zero is true
- `Float` → `0.0` is false; non-zero is true
- `String` → empty string is false; otherwise true
- `Void` (`null` / `void`) → **TypeError** (fail-fast)
- `BoxRef`:
  - bridge boxes only:
    - `BoolBox` / `IntegerBox` / `StringBox` are unboxed and coerced like their primitive equivalents
    - `VoidBox` is treated as `Void` → **TypeError**
  - other BoxRef types → **TypeError**
- `Future` → error (`TypeError`)

This is intentionally fail-fast: “any object is truthy” is **not** assumed by default today.

---

## 4. Operators: `+`, comparisons, equality

### 4.1 `+` (BinaryOp::Add)

Runtime semantics are defined in the Rust VM (`eval_binop` in `src/backend/mir_interpreter/helpers.rs`):

- Numeric addition:
  - `Integer + Integer` → `Integer`
  - `Float + Float` → `Float`
- Numeric promotion:
  - `Integer + Float` / `Float + Integer` → `Float` (promote int→float)
- String concatenation:
  - Decision: accepted (Phase 29bq selfhost unblock; keep fail-fast for Void/Null)
  - `String + <any>` → `String` (right operand is `to_string()`-coerced)
  - `Void`/`Null` on either side → **TypeError** (fail-fast)
  - source-style note: new code should still prefer explicit `x.toString()` when stringify intent matters; broad `"" + x` residue is legacy compatibility still used by selfhost/compiler owners
- Other combinations are `TypeError` (e.g., `Integer + Bool`, `Bool + Bool`, `BoxRef + ...`).
  - Backends that do not implement `String + <any>` must fail-fast with a `TypeError`.

Dev-only note:
- `NYASH_VM_TOLERATE_VOID=1` (or `--dev` paths) may tolerate `Void` in some arithmetic as a safety valve; do not rely on it for spec.

### 4.2 `< <= > >=` (CompareOp)

Runtime semantics (`eval_cmp` in `src/backend/mir_interpreter/helpers.rs`):

- `Integer <=> Integer`
- `Float <=> Float`
- `String <=> String` (lexicographic)
- Other combinations are `TypeError`.

### 4.3 `==` / `!=`

Equality is implemented as `eq_vm` (`src/backend/abi_util.rs`) and used by comparisons:

- Same-kind equality for primitives: `Integer/Float/Bool/String/Void`.
- Cross-kind coercions (Number-only):
  - `Integer` ↔ `Float` only, with a precise rule (avoid accidental true via float rounding)
- `BoxRef == BoxRef` is pointer identity (`Arc::ptr_eq`).
- `Void` is treated as equal to `BoxRef(VoidBox)` and `BoxRef(MissingBox)` for backward compatibility.
- Other mixed kinds are `false` (not an error).

Precise rule for `Int == Float` (or `Float == Int`):
- if Float is NaN → false
- if Float is finite, integral, and exactly representable as i64 → compare as i64
- otherwise → false

---

## 5. `is` / `as` and TypeOp

Source patterns like `x.is("TypeName")` / `x.as("TypeName")` are lowered to MIR `TypeOp(Check/Cast)` (see `src/mir/builder/exprs.rs`).

Runtime behavior (Rust VM):
- `TypeOp(Check, value, ty)` produces a `Bool`.
- `TypeOp(Cast, value, ty)` returns the input value if it matches; otherwise `TypeError`.

Backend note:
- LLVM (llvmlite harness) must match this SSOT; if it differs, treat it as a bug.
- Tracking: Phase 274 P2 (`docs/development/current/main/phases/phase-274/P2-INSTRUCTIONS.md`).

---

## 6. MIR Type Facts (non-semantic metadata)

MIR has a lightweight type vocabulary (`MirType` in `src/mir/types.rs`) and per-value metadata:
- `value_types: ValueId -> MirType` (type annotations / inferred hints)
- `value_origin_newbox: ValueId -> BoxName` (origin facts for “Known receiver”)
- `TypeCertainty::{Known, Union}` used by call routing (`src/mir/definitions/call_unified.rs`)

Important rule:
- These facts are for **optimization/routing** (e.g., Known-only rewrite, callee resolution) and must not be treated as semantic truth.

If you need semantics, define it at the runtime layer (VM) and then optionally optimize by using these facts.

---

## 7. Implementation Anchors (SSOT)

このセクションは「意味論の真実」を実装しているファイルを列挙する。
新しい実装が追加されたらここに追記する（仕様SSOT の分岐を増やさない）。

### 7.1 意味論の実装（正しい形）

| 機能 | ファイル | 関数 | 状態 |
|------|----------|------|------|
| truthiness | `src/backend/abi_util.rs` | `to_bool_vm` | ✅ VMValue 直接 |
| equality | `src/backend/abi_util.rs` | `eq_vm` | ✅ VMValue 直接 |
| type tag | `src/backend/runtime_type_tag.rs` | `tag_from_vmvalue`, `tag_to_str` | ✅ VMValue 直接 |
| type tag (shim) | `src/backend/abi_util.rs` | `tag_of_vm` | ✅ runtime_type_tag.rs 経由 |
| binary ops | `src/backend/mir_interpreter/helpers.rs` | `eval_binop`, `eval_cmp` | ✅ VMValue 直接 |
| arithmetic | `src/backend/mir_interpreter/handlers/arithmetic.rs` | `handle_binop` | ✅ VMValue 直接 |
| is/as type match | `src/backend/runtime_type_spec.rs` | `matches_spec`, `spec_from_mir_type` | ✅ MirType 遮断済み |

### 7.2 移設対象（意味論リーク）

| 機能 | 現在のファイル | 問題 | 移設先 |
|------|---------------|------|--------|
| (なし) | - | - | Phase 15 T4 で完了 |

### 7.3 Phase 15 で追加済み

| 機能 | ファイル | 説明 |
|------|----------|------|
| RuntimeTypeTag | `src/backend/runtime_type_tag.rs` | ✅ 実行時の型タグ enum（VMValue から抽出） |
| RuntimeTypeSpec | `src/backend/runtime_type_spec.rs` | ✅ 意味論 SSOT（type_ops.rs から移設済み） |

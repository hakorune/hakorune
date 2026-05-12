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

### Numeric Substrate Vocabulary (M0)

Decision: accepted for the type-name/storage lock only.

The following fixed-width and pointer-sized integer names are reserved and now
classified by MIR metadata when they appear as type annotation text:

```text
i8 i16 i32 i64 isize
u8 u16 u32 u64 usize
```

Current live semantics are intentionally narrow:

- The parser treats these as ordinary `TYPE_REF` identifiers when they appear
  in annotations.
- Decimal integer literal suffixes for these names are accepted on the Rust
  parser front, for example `0usize`, `1u8`, and `42i64`. They are range-checked
  against exact numeric metadata and preserved as typed integer literal
  metadata, but the emitted runtime value still uses `Integer(i64)`.
- Runtime values still execute on the current dynamic `Integer(i64)` lane.
- The current `>>` operator in that lane is signed i64 arithmetic right shift.
- Typed-object EXE storage planning preserves these names as exact numeric
  storage names in layout plans, while current execution still uses the
  dynamic `Integer(i64)` lane.
- Field, parameter, and accepted return annotations preserve the original
  declared type text in AST metadata so later exact-width rows can refine
  semantics without rediscovering source text. AST JSON and Stage1
  Program(JSON) carry this metadata while keeping names-only `params` for
  compatibility.
- Pointer-sized names resolve their metadata width through the MIR numeric
  target owner (`src/mir/numeric_substrate.rs`). This is target metadata only;
  it does not enable exact `usize` runtime behavior by itself.
- MIR-side exact numeric metadata records source spelling and target-resolved
  signedness/width distinctly from `MirType::Integer`. It is not attached to
  runtime values yet.
- Exact numeric constant metadata and dynamic `Integer(i64)` conversion helpers
  range-check against signedness and resolved width. Typed integer literal
  suffixes publish MIR exact const facts after the same range checks. The MIR
  verifier uses these checks for statically known writes to exact numeric declared fields.
  The verifier also rejects unchecked dynamic writes to exact numeric fields
  whose range does not cover every possible dynamic `Integer(i64)` value.
  Function metadata can carry a `DynamicIntegerRange` runtime-check contract
  for those dynamic field writes. MIR semantic refresh attaches those contracts
  for real exact numeric `FieldSet` producers after optimization and before
  verification. The VM interpreter executes existing contracts at `FieldSet`
  sites and rejects non-integer, negative-unsigned, and out-of-range dynamic
  values before mutation. Param/local verifier checks, contract
  insertion/lowering beyond exact numeric field-write contracts, non-VM backend
  lowering/execution of those contracts, and exact VM runtime values remain
  deferred. Unsupported non-VM backend routes fail fast instead of silently
  dropping exact numeric runtime-check contracts.
- MIR numeric substrate policy now defines checked exact numeric add/sub/mul:
  operands must have the same exact numeric type and results must fit the
  target-resolved range. VM/backend exact arithmetic lowering is still future.
- MIR numeric substrate policy also defines exact numeric comparison and
  unsigned logical right shift. Type mismatch, signed logical shift, and shift
  counts at or above the exact width fail fast. VM/backend lowering remains
  future.
- MIR exact numeric PHI/Select merge policy preserves exact facts only when
  all incoming exact types are identical; exact/dynamic mixes and mismatched
  exact types fail fast in the policy helper.

Deferred and not accepted by this row:

- param/local verifier checks, runtime-check insertion/lowering beyond exact
  numeric field-write contracts, non-VM backend runtime-check lowering,
  exact runtime unsigned range construction, and live unsigned arithmetic
  execution
- route-fact wiring for exact numeric params/locals/control merges
- `u64` values outside signed i64
- live VM/backend exact numeric arithmetic/compare/shift routes and wrapping /
  checked helper-call syntax
- MIR JSON exact-width numeric const tags
- backend/native typed-object slots for exact numeric widths

Backends must not infer exact unsigned or fixed-width behavior from these names
until the corresponding verifier/lowering rows are live.

### Static Const Tables (M11b live)

Decision: the first declaration, read, and narrow integer const-expression rows
are live. Const fn is still reserved.

Accepted first shape:

```hako
static const SIZE_CLASS: u16[] = [
  8 + 8, 3 * 8, 1 << 5, (40 - 8) | 1,
]
```

Current status:

- Rust parser and `.hako` parser accept the first `u16[]` declaration shape.
- The only accepted element type is `u16`.
- Values must evaluate to the `0..65535` range.
- Initializer elements may use integer literals, unary `-`, parentheses, and
  `+`, `-`, `*`, `/`, `%`, `<<`, `>>`, `&`, `|`, `^`.
- The declaration lowers to MIR module metadata `static_data_plans`.
- `NAME[index]` reads from a declared static const table and lowers to MIR
  `StaticDataLoad`.
- Static table reads return current-lane `Integer(i64)` values by zero-extending
  the `u16` element.
- VM execution fail-fasts on negative or out-of-range indices.
- Backends emit readonly data from that plan.
- Runtime `ArrayBox` / `MapBox` construction is not an accepted
  implementation strategy for fixed static tables.

Reserved follow-ups:

- general const expression evaluation outside the narrow `u16[]` initializer
  row
- const references to other declarations
- const fn
- additional element types
- explicit length in the type

Design SSOT:

- `docs/development/current/main/design/static-const-table-syntax-ssot.md`

Low-level capability context:

- `docs/reference/language/low-level-capabilities.md`

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
- canonical enum MIR lowering is now landed too:
  - `EnumCtor` lowers to `VariantMake`
  - `EnumMatch` lowers to `VariantTag` + compare/branch + `VariantProject`
- VM/LLVM fallback runtime semantics are landed on the current route too:
  - variant values use the existing synthetic enum runtime box `__NyVariant_<Enum>` where fallback representation is needed
  - narrow record payloads use synthetic hidden payload boxes `__NyVariantPayload_<Enum>_<Variant>`
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

### Option<T> (public direction, null-free)

`Option<T>` is the public optional-value direction, built on the first-class
enum surface:

```hako
enum Option<T> {
  None
  Some(T)
}
```

Rules:

- `Option::None` is not `null`.
- `Option::Some(null)` is forbidden.
- `Option::Some(void)` is forbidden.
- `Option<T>` is not the Stage0/selfhost compiler helper no-match carrier.

Design SSOT:

- `docs/reference/language/option.md`
- `docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md`

---

## 2. Variables and Re-assignment

- `local x` / `local x = expr` introduces a mutable local variable.
- `local x` is treated as `local x = null` (i.e., a `Void` value) unless an initializer is provided.
- Re-assignment is always allowed: `x = expr`.
- “Immutable locals” (let/const) are not part of the language today; they can be introduced later as lint/strict checks without changing core semantics.

Note: Field type annotations like `field: TypeBox` exist in syntax, but are currently **not enforced** as a type contract (metadata for planner/optimizer/verifier). Stored field initializers (`field = expr` / `field: TypeBox = expr`) run as constructor prologue assignments before the user `birth` body — see `LANGUAGE_REFERENCE_2025.md`.

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

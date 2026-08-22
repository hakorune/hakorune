# Type System (SSOT)

Status: Draft / active
Decision: accepted normative type-law owner

Mutable implementation, carrier, backend-capability, and migration status is
tracked separately in
`docs/development/current/main/workstreams/type-contract-status.md`.

This document defines normative type semantics for Nyash/Hakorune. A backend
that cannot preserve an active semantic contract must reject before effects;
successful execution on another backend is not fallback authority.

Coercion SSOT status:
- Decisions: `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`
- Implemented (VM + LLVM parity): Phase 275 P0

---

## 1. Big Picture

- Nyash is **dynamically typed**: runtime values carry a tag (Integer/Float/Bool/String/Void/BoxRef/Future).
- `local` declares a variable; **re-assignment is allowed** (single keyword policy).
- There is currently no static type checker. Some parts of MIR carry **type facts** as metadata for optimization / routing, not for semantics.

An ordinary call through an exact resolver-proven Dynamic receiver uses the
single selector-independent contract in
[`dynamic-invocation.md`](dynamic-invocation.md). Runtime tags may select
physical storage/drop mechanics, but they do not choose effect, Fault,
suspension, or Home meaning.

### Language v1 annotation guarantee matrix

Decision: accepted.

Canonical `x: T` has one eventual meaning: the value must satisfy semantic
contract `T` at the site's boundary. A metadata-only annotation is an explicit
transitional non-guarantee, not a second meaning for `: T`. Representation,
storage, planner, and Rune facts remain separate axes.

The closed annotation-site vocabulary and implementation projection live in
`src/mir/type_contracts/guarantee_matrix.rs`. Their mutable activation status
belongs to the type-contract status ledger, not this normative document.

At every active site, enforcement is either a runtime check or a fresh
verifier-backed proof. `MirType`, storage/layout metadata, route facts, and
Plan/Rune hints are not semantic proof. Unsupported backends reject before
execution rather than falling back to another backend.

### Record vs Box

Hakorune keeps `record` and `box` as separate source surfaces.

```text
record:
  identity-free value aggregate
  fixed typed data
  replacement via `with`
  no lifecycle / behavior boundary in v0
  no methods / fini / dynamic dispatch in v0

box:
  identity object
  behavior / methods
  mutable state / ownership / lifecycle boundary
```

The Box type surface does not imply that every local name is a strong owner.
Home slots/tokens, ordinary handles, result relations, and explicit Shared
entry are defined independently in `docs/reference/language/ownership.md`.

Use the short rule:

```text
data/value:
  record

thing/owner/behavior/lifecycle:
  box
```

Do not read `record` as a faster `box`.  `record` is the source-level word for
identity-free named data.  Optimization is handled by the compiler's aggregate
and object storage plans where proofs allow it.

Examples:

```hako
record Point {
    x: i64
    y: i64
}

box Counter {
    value: i64 = 0

    inc(delta: i64): void {
        me.value = me.value + delta
    }
}
```

`with` is record-only:

```hako
local moved = point with { x: point.x + 1 }
```

Record field annotations are semantic contracts at both construction and
`with` replacement boundaries. Explicit field expressions evaluate exactly
once in source order; missing defaults evaluate exactly once in declaration
order. Each active field contract is checked immediately after its final value
is produced, and the record value is published only after all checks pass.
`with` produces a replacement and never mutates its base value. Storage layout,
`MirType`, and packed-record plans cannot prove these contracts.

Ordinary boxes do not support `with` copy/update semantics.

Design SSOT:

- `docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md`
- `docs/development/current/main/design/object-storage-plan-boundary-ssot.md`
- `docs/development/current/main/phases/phase-296x/296x-734-AGG-STORAGE-PLAN-000.md`

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
- Decimal integer literal suffixes such as `0usize`, `1u8`, and `42i64` are
  historical Rust-parser evidence only and are rejected by both Language v1
  grammar profiles. Exact numeric contracts come from declaration annotations
  plus ordinary literals or dynamic values; suffix spelling is not authority.
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
- Return type annotation is syntactically optional. Omission means an
  unannotated result contract; it is neither an implicit `void` contract nor
  source-level return-type inference. Explicit `: void` declares a no-value
  contract, and `void` remains valid inside generic types such as
  `Result<void, Error>`. Function fallthrough, explicit-return, and physical
  signature planning are owned by
  `docs/reference/language/function-exit-and-entry-result.md`.
- Pointer-sized names resolve their metadata width through the MIR numeric
  target owner (`src/mir/numeric_substrate.rs`). This is target metadata only;
  it does not enable exact `usize` runtime behavior by itself.
- MIR-side exact numeric metadata records source spelling and target-resolved
  signedness/width distinctly from `MirType::Integer`. It is not attached to
  runtime values yet.
- Exact numeric constant metadata and dynamic `Integer(i64)` conversion helpers
  range-check against signedness and resolved width. Builder-owned exact const
  facts may still exist for imported/historical AST evidence, but canonical
  source does not create them through literal suffixes. The MIR verifier checks
  statically known writes using a structural proof and rejects every unchecked
  dynamic exact-numeric field write. Even `i64` needs a runtime type check when
  the incoming boundary has no active parameter/local proof.
  Function metadata can carry a `DynamicIntegerRange` runtime-check contract
  for those dynamic field writes. MIR semantic refresh attaches those contracts
  for real exact numeric `FieldSet` producers after optimization and before
  verification. The VM interpreter executes existing contracts at `FieldSet`
  sites and rejects non-integer, negative-unsigned, and out-of-range dynamic
  values before mutation. Param/local verifier checks, contract
  insertion/lowering beyond exact numeric field-write contracts, backend
  lowering/execution of those contracts, and exact VM runtime values remain
  deferred. Unsupported non-VM backend routes fail fast instead of silently
  dropping exact numeric runtime-check contracts, exact numeric typed-object
  storage, or exact numeric operation route facts.
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
  numeric field-write contracts, backend exact numeric lowering/execution,
  exact runtime unsigned range construction, and product-backend unsigned
  arithmetic execution
- route-fact wiring for exact numeric params/locals/control merges
- `u64` values outside signed i64
- product-backend exact numeric arithmetic/compare/shift routes and wrapping /
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

Accepted first-row contract:

- Rust parser and `.hako` parser accept the first `u16[]` declaration shape.
- The only accepted element type is `u16`.
- Values must evaluate to the `0..65535` range.
- Initializer elements may use integer literals, unary `-`, parentheses, and
  `+`, `-`, `*`, `/`, `%`, `<<`, `>>`, `&`, `|`, `^`.
- The declaration publishes a source-owned Static Table contract spec. MIR
  `static_data_plans` are a derived readonly representation of that spec.
- Semantic refresh must prove that the source spec, derived plan, and every
  `StaticDataLoad` agree before verifier, JSON, VM, or backend consumption.
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
- At a function or Script result boundary, explicit `void` contributes Unit
  with explicit-void provenance. The boundary rules and provenance vocabulary
  are owned by `function-exit-and-entry-result.md`; this section continues to
  own the current `null`/`void` type relation.

Practical consequence:
- `x == null` and `x == void` are equivalent checks.
- `WeakRef.weak_to_strong()` returns `null` on failure (i.e., `void` / none).
- `void` is also an accepted type annotation token. `fn(): void` and
  `fn(): Result<void, Error>` preserve `"void"` in AST/MIR metadata. Omitting
  the annotation instead records an unannotated result contract; it does not
  declare `void`.

### Option<T> / Result<T,E> (current enum prelude, null-free)

`Option<T>` and `Result<T,E>` are public enum prelude surfaces:

```hako
enum Option<T> {
  None
  Some(T)
}

enum Result<T, E> {
  Ok(T)
  Err(E)
}
```

Rules:

- constructors use `Type::Variant`, for example `Option::None` and
  `Result::Ok(value)`;
- dot variants such as `Result.Ok(value)` are rejected for known enum variants;
- prelude `Option<T>` / `Result<T,E>` local constructors need explicit typed
  context when generic parameters would otherwise be ambiguous;
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
- An active non-optional exact contract requires an initializer. The general
  unannotated-local absence rule is decided by the Failure/Outcome row.
- Re-assignment is always allowed: `x = expr`.
- “Immutable locals” (let/const) are not part of the language today; they can be introduced later as lint/strict checks without changing core semantics.

Stored field initializers (`field = expr` / `field: TypeBox = expr`) run as
constructor prologue assignments before the user `birth` body. An active field
annotation is a semantic contract, while storage and planner projections remain
derived facts. See `docs/reference/language/lifecycle.md`.

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

The normative Dynamic operand/outcome/lifecycle boundary is defined in
[dynamic-operators.md](dynamic-operators.md). The Rust VM (`eval_binop` in
`src/backend/mir_interpreter/helpers.rs`) is implementation evidence and must
conform to that contract; it is not the semantic authority.

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

The profile-neutral Dynamic comparison contract is defined in
[dynamic-operators.md](dynamic-operators.md). Runtime behavior in `eval_cmp`
is conformance evidence:

- `Integer <=> Integer`
- `Float <=> Float`
- `String <=> String` (lexicographic)
- Other combinations are `TypeError`.

### 4.3 `==` / `!=`

Equality is implemented as `eq_vm` (`src/backend/abi_util.rs`) and used by comparisons:

- Same-kind equality for primitives: `Integer/Float/Bool/String/Void`.
- `String == String` is exact Unicode scalar-value sequence equality: the
  sequences must have the same length and scalar order. It is
  case-sensitive, performs no normalization, and is locale/collation-free.
- The logical `Text` class used by the typed Loop Recipe follows this same
  content law. A `StringBox` participates in that law only when an explicit
  source contract admits the `StringBox-as-Text` bridge; this does not change
  ordinary `BoxRef` equality.
- `String != String` is the logical negation of the same String/Text equality.
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

If a semantic row is missing, define it in the owning language contract and
then verify runtime/backend conformance. Do not infer language meaning from MIR
facts or the VM implementation.

---

## 7. Implementation Navigation

Mutable implementation anchors and migration debt are listed in
`docs/development/current/main/workstreams/type-contract-status.md`. Those
paths are navigation evidence and do not override this type law.

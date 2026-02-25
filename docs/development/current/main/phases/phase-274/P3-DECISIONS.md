# Phase 274 P3 (decision): Coercion SSOT (truthiness / `==` / `+`)

Status: accepted (2025-12-22) / pending implementation

This document freezes **what coercions mean** at the language level, so runtime behavior cannot “emerge” from resolver/type-facts.

SSOT anchor (current executable behavior): `docs/reference/language/types.md`  
Phase overview: `docs/development/current/main/phases/phase-274/README.md`
Implementation phase: `docs/development/current/main/phases/phase-275/README.md`

---

## 1) Terms

In this doc, “coercion” means: when operands/types differ, do we:
- convert implicitly,
- return a deterministic result (e.g. `false`),
- or fail-fast (`TypeError`)?

Target constraints:
- Fail-Fast where it prevents silent bugs
- No “JS-style surprise coercion”
- Dynamic runtime remains (no static type system required)
- Backend parity (VM/LLVM) or explicit divergence, never accidental drift

---

## 2) Proposed SSOT (recommended)

Based on the project philosophy, the recommended SSOT choice is:
- **truthiness: A1**
- **`==`: B2 (Number-only)**
- **`+`: C2 (Number-only promotion)**

The sections below define each choice precisely.

---

## 3) truthiness (boolean context)

### Decision: A1 (Fail-Fast)

`Void` in condition is **TypeError**.

Allowed in boolean context:
- `Bool` → itself
- `Integer` → `0` false, non-zero true
- `Float` → `0.0` false, non-zero true
- `String` → empty false, otherwise true

Disallowed (TypeError):
- `Void` (always error)
- `BoxRef` (by default)
  - Exception: only **explicit bridge boxes** may be unboxed to the corresponding primitive for truthiness:
    - `BoolBox` / `IntegerBox` / `StringBox`
  - `VoidBox` is treated as `Void` → TypeError

Recommended explicit patterns:
- existence check: `x != Void`
- type check: `x.is("T")` / `x.as("T")`
- explicit conversion (if we add it): `bool(x)` (but `bool(Void)` remains TypeError)

Implementation impact (where to change):
- Rust VM: `src/backend/abi_util.rs::to_bool_vm`
- LLVM harness: must match the VM semantics used for branch conditions

---

## 4) `==` (equality)

### Decision: B2 (Number-only)

Rules:
- Same-kind primitives compare normally.
- `Int` ↔ `Float` comparisons are allowed (Number-only).
- `Bool` is **not** a number: `Bool` ↔ `Int/Float` has no coercion.
- Other mixed kinds: deterministic **`false`** (not an error).
- `BoxRef == BoxRef`: identity only.

#### Precise rule for `Int == Float` (avoid “accidental true”)

To avoid float rounding making `true` incorrectly:

For `Int == Float` (or `Float == Int`):
1) If Float is NaN → `false`
2) If Float is finite, integral (fractional part is 0), and within `i64` exact range:
   - convert Float → Int exactly, then compare Ints
3) Otherwise → `false`

Migration note:
- If legacy behavior existed for `1 == true`, prefer a transition phase where it becomes **TypeError first** (to surface bugs), then settle to `false` if desired.

Implementation impact:
- Rust VM: `src/backend/abi_util.rs::eq_vm` (and any helpers)
- LLVM harness: must mirror the same decision for `compare ==` lowering

---

## 5) `+` (add / concat)

### Decision: C2 (Number-only promotion)

Rules:
- `Int + Int` → `Int`
- `Float + Float` → `Float`
- `Int + Float` / `Float + Int` → `Float` (promote Int→Float)
- `String + String` → concat
- `String + non-string` / `non-string + String` → **TypeError** (no implicit stringify)
- Other combos → TypeError

Implementation impact:
- Rust VM: `src/backend/mir_interpreter/helpers.rs::eval_binop` (BinaryOp::Add)
- LLVM harness: binop `+` lowering must follow the same coercion rules

---

## 6) Minimum test matrix (SSOT lock)

### 6.1 truthiness

- Bool: `if true`, `if false`
- Int: `if 0`, `if 1`, `if -1`
- Float: `if 0.0`, `if 0.5`, `if NaN` (define if NaN counts as truthy)
- String: `if ""`, `if "a"`
- Void: `if Void` → TypeError (A1)
- BoxRef:
  - bridge: `BoolBox(true)`, `IntegerBox(0)`, `StringBox("")`
  - non-bridge: `Foo()` → TypeError

### 6.2 equality

- same-kind primitives
- Int↔Float:
  - `1 == 1.0` true
  - `1 == 1.1` false
  - `NaN == NaN` false
- Bool↔Int:
  - `true == 1` (explicitly decide: TypeError during migration vs final false)
- BoxRef identity:
  - same handle true, different handles false

### 6.3 plus

- Int/Float add
- Int+Float promotion
- String+String concat
- String mixed TypeError

---

## 7) Migration plan (if changing behavior)

Recommended two-step approach:

1) Compatibility freeze (Phase 274)
- Document current behavior (already in `types.md`)
- Add warnings / diagnostics where possible (no new env sprawl)

2) Switch semantics (Phase 275 or later)
- Implement A1/B2/C2 in VM and LLVM
- Add fixtures to lock the SSOT
- Ensure error messages provide “fix-it” guidance (`str(x)`, `x != Void`, etc.)

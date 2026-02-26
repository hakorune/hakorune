# Phase 274 P2 (impl): LLVM (llvmlite harness) TypeOp alignment

Status: planned / design-first

Goal: make LLVM harness execution match the SSOT semantics in `docs/reference/language/types.md` for:
- `TypeOp(Check, value, ty)` → `Bool`
- `TypeOp(Cast, value, ty)` → `value` or `TypeError`

Primary reference implementation (SSOT runtime): `src/backend/mir_interpreter/handlers/type_ops.rs`

---

## 0. What is currently wrong (must-fix)

LLVM harness TypeOp is stubbed in `src/llvm_py/instructions/typeop.py`:
- `is`: returns 0 for most types (IntegerBox is “non-zero” heuristic)
- `cast/as`: pass-through (never errors)

This conflicts with SSOT:
- `is` must reflect actual runtime type match.
- `as` must fail-fast (`TypeError`) on mismatch.

Note:
- It is OK if the compiler constant-folds trivial cases (e.g. `1.is("Integer")`).
- For P2 verification, you must use a fixture that keeps `TypeOp` in MIR (runtime-unknown / union value).

---

## 1. Acceptance criteria (minimum)

1) Behavior parity with Rust VM (SSOT)
- With `NYASH_LLVM_USE_HARNESS=1` and `--backend llvm`, this fixture behaves the same as VM:
  - `apps/tests/phase274_p2_typeop_primitives_only.hako` (recommended: harness-safe baseline)

2) Fail-fast
- `as` on mismatch must raise a TypeError (not return 0 / pass-through).
- `is` must return `0/1` deterministically (no “unknown → 0” unless it is truly not a match).

3) No hardcode / no new env sprawl
- No “BoxName string match special-cases” except small alias normalization shared with frontend (`IntegerBox`/`StringBox` etc.).
- Do not add new environment variables for behavior.

---

## 2. Design constraint: LLVM harness value representation (key risk)

In llvmlite harness, a runtime “value” is currently represented as an `i64`, but it mixes:
- raw integers (from `const i64`)
- boxed handles (e.g. strings are boxed to handles via `nyash.box.from_i8_string`)
- various call/bridge conventions

Because a handle is also an `i64`, **the harness cannot reliably decide at runtime** whether an `i64` is “raw int” or “handle”, unless the value representation is made uniform.

This means TypeOp parity cannot be achieved reliably without addressing representation.

---

## 3. Recommended implementation strategy (P2): make representation uniform for TypeOp

### Strategy A (recommended): “all values are handles” in LLVM harness

Make every runtime value in llvmlite harness be a handle (i64) to a boxed value:
- integers: `nyash.box.from_i64(i64) -> handle`
- floats: `nyash.box.from_f64(f64) -> handle`
- strings: already boxed (`nyash.box.from_i8_string`)
- bool/void: use existing conventions (or add kernel shims if needed)

Then TypeOp becomes implementable via runtime introspection on handles.

#### A.1 Kernel helper needed (small, SSOT-friendly)

Add a kernel export (in `crates/nyash_kernel/src/lib.rs`) that checks a handle’s runtime type:
- `nyash.any.is_type_h(handle: i64, type_name: *const i8) -> i64` (0/1)
- optionally `nyash.any.cast_h(handle: i64, type_name: *const i8) -> i64` (handle or 0; but prefer fail-fast at caller)

Implementation rule:
- Must use actual runtime object type (builtins + plugin boxes + InstanceBox class name).
- Must not guess via resolver/type facts.

#### A.2 LLVM harness lowering

Update `src/llvm_py/instructions/typeop.py`:
- Always resolve `src_val` as handle (`i64`).
- `check/is`: call `nyash.any.is_type_h(src_val, type_name_ptr)` → i64 0/1
- `cast/as`: call `is_type_h`; if false, emit a runtime error (use existing “panic”/error path if available) or call a kernel `nyash.panic.type_error` style function (add if missing).

Also update other lowerers incrementally so that values feeding TypeOp are handles (start with the fixture path).

### Strategy B (fallback): keep mixed representation, but document divergence (not parity)

If Strategy A is too large for P2, constrain scope:
- Implement `TypeOp` using compile-time `resolver.value_types` hints.
- Document clearly in Phase 274 README: LLVM harness TypeOp is “best-effort using type facts” and is not SSOT-correct under re-assignment.

This keeps the harness useful for SSA/CFG validation, but is not runtime-parity.

Note: Strategy B should be treated as temporary and must be called out as backend divergence in docs.

---

## 4. Concrete work items (P2)

1) Audit current failure path
- Identify how LLVM harness reports runtime errors today (type errors, asserts).
- Prefer a single runtime helper rather than sprinkling Python exceptions.

1.5) Fix MIR JSON emission for TypeOp (required)

The LLVM harness consumes MIR JSON emitted by the Rust runner.
If `TypeOp` is missing in that JSON, the harness will never see it (and the JSON can become invalid due to missing defs).

Checklist:
- `src/runner/mir_json_emit/mod.rs` must emit `{"op":"typeop", ...}` in **both** emitters:
  - `emit_mir_json_for_harness` (nyash_rust::mir) ✅ already supports TypeOp
  - `emit_mir_json_for_harness_bin` (crate::mir) ⚠️ ensure TypeOp is included

2) Add kernel introspection helper(s)
- `crates/nyash_kernel/src/lib.rs`: add `nyash.any.is_type_h`.
- It must handle:
  - primitives boxed (`IntegerBox`, `FloatBox`, `BoolBox`, `StringBox`, `VoidBox`)
  - `InstanceBox` user classes (by `class_name`)
  - plugin boxes (by metadata / resolved type name)

3) Implement real `TypeOp` lowering
- `src/llvm_py/instructions/typeop.py`:
  - normalize `target_type` aliases (same mapping as frontend docs: `Int` → `IntegerBox`, etc.)
  - `is` → call kernel check
  - `as`/`cast` → check then return src or TypeError

4) Add LLVM smoke (integration)
- New script (name suggestion):
  - `tools/smokes/v2/profiles/integration/apps/archive/phase274_p2_typeop_is_as_llvm.sh`
- Run:
  - `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/phase274_p2_typeop_primitives_only.hako`
- Expect: exit code `3` (same as VM).

---

## 5. Notes / non-goals (P2)

- Do not implement a full static type system here.
- Do not add rule logic to the resolver (no “guessing chains”).
- Do not add new environment variables for behavior selection.
- If you must limit scope, limit it by fixtures and document it in Phase 274 README as explicit divergence.

### Fixture rule (important)

To avoid “TypeOp disappeared” false negatives:
- Do not use pure compile-time constants for `is/as` checks.
- Prefer a union value formed by a runtime-unknown branch (e.g. `process.argv().size() > 0`).
  - Note: `env.process.argv` is currently not supported on Rust VM, and `env.get` is not linked for LLVM AOT yet; keep harness fixtures minimal unless the required externs are implemented in NyRT.

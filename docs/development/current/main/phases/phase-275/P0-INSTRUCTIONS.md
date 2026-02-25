# Phase 275 P0 (impl): Coercion SSOT rollout

Status: planned / implementation guide

This is the “next instruction sheet” for implementing the accepted coercion SSOT (A1/B2/C2) across backends.

SSOT decisions:
- `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`

---

## Scope (P0)

Implement and lock these three rule sets:
- truthiness: `Void`/`BoxRef` fail-fast (with bridge-box exceptions)
- equality: B2 (Number-only, precise Int↔Float)
- `+`: C2 (Number-only promotion; String+String only; String mixed → TypeError)

Backends in scope:
- Rust VM (primary SSOT)
- LLVM harness (llvmlite path) parity with Rust VM

Out of scope:
- adding new language features (keywords)
- expanding env var toggles
- rewriting the optimizer broadly (only touch what is required to enforce semantics)

---

## Step 0: Lock reference + plan

- Keep `docs/reference/language/types.md` as “current executable SSOT” until the implementation is complete.
- After implementation + tests land, update `types.md` to the new semantics (it becomes SSOT again).

---

## Step 1: Rust VM — truthiness (A1)

Target:
- `src/backend/abi_util.rs::to_bool_vm` (or equivalent truthiness entry)

Changes:
- `Void` in boolean context → return `TypeError`
- `BoxRef`:
  - allow only explicit bridge boxes: BoolBox/IntegerBox/StringBox
  - treat VoidBox as Void (→ TypeError)
  - other BoxRef types → TypeError

Acceptance:
- A dedicated fixture demonstrates `if Void { ... }` is a runtime error (fail-fast).

---

## Step 2: Rust VM — equality (B2)

Target:
- `src/backend/abi_util.rs::eq_vm` (and any helpers it relies on)

Changes:
- Remove Bool↔Int coercion.
- Keep Int↔Float comparison, but make it precise:
  - if Float is NaN → false
  - if Float is integral and within i64 exact range → compare as Int exactly
  - otherwise → false
- Mixed kinds (except Int↔Float) → false (not error).
- BoxRef equality stays identity.

Acceptance:
- Tests cover:
  - `1 == 1.0` true
  - `1 == 1.1` false
  - `true == 1` false (or TypeError if you choose a transition rule; document explicitly)

---

## Step 3: Rust VM — `+` (C2)

Target:
- `src/backend/mir_interpreter/helpers.rs::eval_binop` for `BinaryOp::Add`

Changes:
- Numeric:
  - Int+Int → Int
  - Float+Float → Float
  - Int+Float / Float+Int → Float (Int promoted to Float)
- String:
  - String+String → concat
  - String mixed → TypeError (no implicit stringify)
- Everything else → TypeError

Acceptance:
- Tests cover:
  - `1 + 2.0` → `3.0`
  - `"a" + "b"` → `"ab"`
  - `"a" + 1` → TypeError

---

## Step 4: LLVM harness parity

Targets (likely):
- `src/llvm_py/instructions/binop.py` for `+`
- `src/llvm_py/instructions/compare.py` for `==`
- truthiness for branch conditions (inspect branch lowering):
  - `src/llvm_py/instructions/controlflow/branch.py`
  - and any “truthy” conversions used in control-flow lowering

Notes:
- LLVM harness uses MIR JSON metadata (`value_types`) for discriminating raw vs handle.
- Keep behavior identical to Rust VM; do not re-introduce “string mixed concat”.

Acceptance:
- VM and LLVM smokes use the same fixtures and produce identical exit codes.

---

## Step 5: Fixtures + smoke tests (SSOT lock)

Create minimal, self-contained fixtures under `apps/tests/` and add smokes under `tools/smokes/v2/profiles/integration/apps/`.

Suggested fixtures (names; adjust as needed):
- `apps/tests/phase275_p0_truthiness_void_error_min.hako`
- `apps/tests/phase275_p0_eq_number_only_min.hako`
- `apps/tests/phase275_p0_plus_number_only_min.hako`

Smoke targets:
- VM: `..._vm.sh`
- LLVM: `..._llvm.sh` (harness/EXE path consistent with current infra)

Rules:
- No reliance on hidden env toggles.
- If a test needs runtime-unknown values, avoid externs that aren’t supported on VM/LLVM.

---

## Step 6: Update SSOT docs

After the implementation is complete and tests pass:
- Update `docs/reference/language/types.md` to the new semantics:
  - truthiness: Void/BoxRef fail-fast
  - equality: B2
  - `+`: C2
- Update Phase 274/275 status:
  - `docs/development/current/main/phases/phase-274/README.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/30-Backlog.md`


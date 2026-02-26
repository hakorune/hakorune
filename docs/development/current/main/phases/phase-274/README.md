# Phase 274 (active): Type SSOT Alignment (local + dynamic runtime)

Status: active / design-first

Goal: make the **language-level type semantics** and the **runtime behavior** consistent and easy to reason about, without turning Nyash into a statically typed language.

This phase is about:
- clarifying SSOT docs,
- closing “frontend emits it but VM can’t run it” gaps,
- and preventing “type facts / resolver guessing” from silently becoming language semantics.

---

## Background (why this exists)

Current state:
- Nyash is dynamic at runtime (VM executes tagged values).
- MIR builder attaches type metadata (`value_types`, `value_origin_newbox`) for routing/optimization.
- Some docs describe stricter semantics than the VM actually implements.
- `TypeOp` is emitted by the frontend for `is/as`.

Problems:
- Spec drift: quick docs vs runtime behavior differ (truthiness / equality / compare / `+`).
- Capability drift across backends: “VM is correct” but “LLVM harness differs” (TypeOp).
- Type metadata risks becoming implicit semantics via resolver fallback chains.

SSOT decisions should be expressed in:
- language docs (meaning),
- runtime (execution),
- and only then optimization facts (rewrite / routing).

---

## SSOT references

- Language type semantics (SSOT): `docs/reference/language/types.md`
- VM semantics source: `src/backend/abi_util.rs`, `src/backend/mir_interpreter/helpers.rs`
- MIR type vocabulary: `src/mir/types.rs`
- Call certainty vocabulary: `src/mir/definitions/call_unified.rs`

---

## Scope (P0/P1/P2/P3)

### P0 (docs-only): establish SSOT and remove contradictions

Deliverables:
- `docs/reference/language/types.md` (SSOT)
- Quick-reference points to SSOT and stops contradicting runtime

Acceptance:
- docs no longer claim semantics that the Rust VM clearly violates.

### P1 (impl): make `TypeOp` runnable on Rust VM

Goal:
- `x.is("T")` / `x.as("T")` lowering exists already; make it executable on the primary backend (Rust VM).

Acceptance (minimum):
- Rust VM implements `MirInstruction::TypeOp { op: Check|Cast, value, ty }`
- Add a small executable fixture/smoke that exercises `is/as`
- No new env vars; fail-fast errors on unsupported casts/checks

Implementation guide:
- `docs/development/current/main/phases/phase-274/P1-INSTRUCTIONS.md`

Status: ✅ done (2025-12-22)

Artifacts:
- Fixture: `apps/tests/phase274_p1_typeop_is_as_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase274_p1_typeop_is_as_vm.sh`
- VM handler: `src/backend/mir_interpreter/handlers/type_ops.rs`

### P2 (impl): align LLVM (llvmlite harness) `TypeOp` to SSOT

Goal:
- Make LLVM harness behavior match Rust VM (SSOT) for `TypeOp(Check/Cast)`.

Current mismatch:
- `src/llvm_py/instructions/typeop.py` is stubbed:
  - `is` returns 0 for most types (special-cases `IntegerBox` as “non-zero”)
  - `cast/as` are pass-through

Acceptance (minimum):
- With `NYASH_LLVM_USE_HARNESS=1` + `--backend llvm`, the P1 fixture has the same observable result as Rust VM.
- Unsupported cases fail-fast (TypeError), not silent 0/“passthrough”.
- No new environment-variable toggles; differences must be fixed or explicitly documented.

Implementation guide:
- `docs/development/current/main/phases/phase-274/P2-INSTRUCTIONS.md`

Status: ✅ done (2025-12-22)

Artifacts:
- Kernel type check helper: `crates/nyash_kernel/src/lib.rs` (`nyash.any.is_type_h`)
- LLVM TypeOp lowering: `src/llvm_py/instructions/typeop.py`
- MIR JSON emission fix (bin): `src/runner/mir_json_emit/mod.rs` (emit `op:"typeop"`)
- Fixture (LLVM-safe): `apps/tests/phase274_p2_typeop_primitives_only.hako`
- Smoke (LLVM): `tools/smokes/v2/profiles/integration/apps/archive/phase274_p2_typeop_is_as_llvm.sh`

### P3 (decision + optional impl): tighten or document coercions

Decision points to settle (SSOT):
- Truthiness for arbitrary BoxRef (allow “any object is truthy” vs fail-fast)
- Equality cross-coercions (`int↔bool`, `int↔float`) — keep, restrict, or gate behind a profile
- `+` mixed numeric types (`int+float`) — keep TypeError or add explicit conversions

Acceptance:
- whichever behavior is chosen becomes consistent across backends and docs.

Decision memo (P3):
- `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`

Status:
- P3 decisions are ✅ accepted; implementation is tracked in Phase 275.

---

## Non-goals

- Full static typing / inference engine
- Widening the language surface area (new keywords) as the first move
- Adding more environment-variable toggles as a long-term solution

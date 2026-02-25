# Phase 276 P0: Quick wins (LLVM harness maintainability)

Status: ✅ completed (2025-12-22)

This sheet documents what Phase 276 P0 targeted (and can be used to reproduce the intent for future refactors).

Reference completion:
- `docs/development/current/main/phases/phase-276/P0-COMPLETION.md`

Non-goals:
- pipeline unification / order drift fixes (tracked as Phase 279)
- new language features
- new env vars

---

## Task 1: Remove noisy debug leftovers

Target:
- `src/llvm_py/phi_wiring/wiring.py`

Acceptance:
- no stacktrace spam in normal runs

---

## Task 2: Consolidate PHI dst type lookup into SSOT

Create:
- `src/llvm_py/phi_wiring/type_helper.py`

Provide:
- `get_phi_dst_type(...)` (SSOT for “what is the intended type of this ValueId?”)
- `dst_type_to_llvm_type(...)` (SSOT for MIR dst_type → LLVM IR type)

Integrate (remove duplicate logic):
- `src/llvm_py/phi_wiring/tagging.py`
- `src/llvm_py/llvm_builder.py`
- `src/llvm_py/phi_wiring/wiring.py`

Acceptance:
- the same ValueId gets the same effective type across these entry points
- adding a new MIR dst_type requires changing only `type_helper.py`

---

## Task 3: Make PHI type mismatch visible

Target:
- `src/llvm_py/phi_wiring/wiring.py`

Policy:
- important mismatch warnings should be visible even without debug env vars
- keep the full trace behind the existing debug gate

---

## Task 4: Confirm the change does not regress existing smokes

Minimum:
- build still succeeds
- representative LLVM harness runs still pass (no new failures)

Note:
- If a failure points to “two pipelines / ordering drift”, treat it as Phase 279 scope.

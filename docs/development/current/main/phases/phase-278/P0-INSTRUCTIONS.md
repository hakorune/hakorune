# Phase 278 P0: Remove deprecated PHI debug env vars

Status: planned / cleanup

Goal: remove legacy PHI-related environment variables that were consolidated in Phase 277 P2, so the ecosystem converges to a single set of PHI debug knobs.

SSOT references:
- Consolidation completion: `docs/development/current/main/phases/phase-277/P2-COMPLETION.md`
- Env var reference (current): `docs/reference/environment-variables.md`

Target SSOT (post-Phase 278):
- `NYASH_LLVM_DEBUG_PHI=1`
- `NYASH_LLVM_DEBUG_PHI_TRACE=1`
- `NYASH_LLVM_PHI_STRICT=1`

Non-goals:
- introduce new debug toggles
- change PHI behavior (only remove deprecated inputs)
- change CLI defaults

---

## 1) Identify deprecated env vars (remove inputs)

Remove support for the legacy variables that were “merged” in Phase 277 P2.

Expected deprecated set (verify in `docs/reference/environment-variables.md`):
- `NYASH_LLVM_PHI_DEBUG`
- `NYASH_PHI_TYPE_DEBUG`
- `NYASH_PHI_ORDERING_DEBUG`
- `NYASH_LLVM_TRACE_PHI`
- `NYASH_LLVM_VMAP_TRACE`
- `NYASH_PYVM_DEBUG_PHI` (if still present in code; PyVM line is historical)

Policy:
- Do not keep “silent compatibility”.
- If a deprecated var is detected, print a one-line error with the replacement and exit non-zero.

Rationale:
- Keeping deprecated behavior is how env var sprawl comes back.

---

## 2) Update the runtime checks to accept only the SSOT set

Target:
- `src/llvm_py/phi_wiring/debug_helper.py` (SSOT entry for PHI debug flags)

Acceptance:
- only the SSOT variables are read
- deprecated variables trigger a clear failure message (replacement shown)

---

## 3) Update documentation (SSOT)

Update:
- `docs/reference/environment-variables.md`

Requirements:
- Remove deprecated entries from tables (or move them to a “Removed in Phase 278” section).
- Keep examples only with `NYASH_LLVM_DEBUG_PHI`, `NYASH_LLVM_DEBUG_PHI_TRACE`, `NYASH_LLVM_PHI_STRICT`.
- Add a short migration note:
  - “If you used X, replace with Y.”

---

## 4) Add/Update smoke coverage

Minimum:
- One LLVM harness smoke that runs with:
  - `NYASH_LLVM_DEBUG_PHI=1`
  - `NYASH_LLVM_DEBUG_PHI_TRACE=1`
  - `NYASH_LLVM_PHI_STRICT=1`
  and verifies the run completes (no strict-mode violations in the fixture).

Deprecation enforcement:
- One smoke (or a small shell snippet in an existing test) that sets a deprecated var and expects a non-zero exit.

No new env vars.

---

## 5) Completion criteria

- Deprecated env vars no longer affect behavior (removed from code paths).
- Deprecated env vars cause a fail-fast error with a replacement hint.
- Docs reflect only the SSOT set.
- The representative smokes remain green.

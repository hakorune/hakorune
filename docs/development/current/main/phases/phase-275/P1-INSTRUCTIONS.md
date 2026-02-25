# Phase 275 P1 (docs/lock): Update types.md + lock coercion matrix

Status: planned / instruction sheet

This is the follow-up after Phase 275 P0 (implementation) lands.

Prereq:
- Phase 275 P0 is complete and VM/LLVM smokes are green.
- Decisions are frozen in `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`.

---

## 1) Update the language SSOT doc

File:
- `docs/reference/language/types.md`

Edits:
- Move “Accepted (Phase 275)” rules from “pending” into the **current executable SSOT** sections:
  - truthiness: `Void` and non-bridge `BoxRef` become `TypeError`
  - `==`: B2 (Number-only) with precise Int↔Float; Bool↔Number has no coercion
  - `+`: C2 (Number-only promotion) + String+String only; String mixed `TypeError`
- Add a short “Migration notes” subsection:
  - how to rewrite old code (`x != Void`, `str(x)`, interpolation)
  - explicitly call out any intentionally-breaking changes

Acceptance:
- `types.md` no longer describes the legacy behavior (e.g. `Void -> false` in conditions, `"a"+1` concat).

---

## 2) Add the minimum test matrix fixtures (SSOT lock)

Goal:
- prevent semantic drift by locking the truthiness / `==` / `+` matrix in fixtures + smokes.

Add fixtures under `apps/tests/` (minimal, self-contained):
- `apps/tests/phase275_p0_truthiness_min.hako`
- `apps/tests/phase275_p0_eq_min.hako`
- `apps/tests/phase275_p0_plus_min.hako`

Rules:
- no env toggles required
- no dependency on unsupported externs/symbols on LLVM line

Add smokes under `tools/smokes/v2/profiles/integration/apps/`:
- `phase275_p0_truthiness_vm.sh` / `phase275_p0_truthiness_llvm.sh`
- `phase275_p0_eq_vm.sh` / `phase275_p0_eq_llvm.sh`
- `phase275_p0_plus_vm.sh` / `phase275_p0_plus_llvm.sh`

Acceptance:
- all 6 smokes pass (expected exit codes fixed and documented in the scripts).

---

## 3) Update “Now / Backlog”

Files:
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/phases/phase-275/README.md`

Edits:
- Mark Phase 275 as ✅ complete (P0 done; P1 done).
- Move any remaining work (warnings/lints, broader coercion coverage) into Phase 276+ entries.

---

## 4) Optional: add diagnostics without new env vars (future phase)

If you want to surface legacy patterns proactively (without changing semantics again), create a new phase and keep it strictly “diagnostic-only”:
- warnings for `if Void`, string-mixed `+`, etc.
- no behavior toggles via new env vars


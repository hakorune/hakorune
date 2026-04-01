# Phase 276 P0: Quick wins (type helper SSOT)

Status: ✅ completed (2025-12-22)

Goal: keep the LLVM harness maintainable by removing “local heuristics” and consolidating type lookup into a single SSOT helper.

Scope (P0):
- Consolidate PHI destination type lookup into `type_helper.py` (SSOT).
- Remove noisy debug leftovers.
- Make “PHI type mismatch” more visible (fail-fast friendly diagnostics).

Docs:
- Instructions: `docs/development/current/main/phases/phase-276/P0-INSTRUCTIONS.md`
- Completion: `docs/development/current/main/phases/phase-276/P0-COMPLETION.md`

Note (important):
- The “two pipelines / order drift” root cause is tracked separately as **Phase 279** (pipeline SSOT unification).

Non-goals:
- introduce new language features
- widen the type system (no Union/Any work here)
- broad optimizer rewrite

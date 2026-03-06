---
Status: SSOT
Scope: JoinIR planner-required (Phase 29bq and friends)
Related:
- Debug workflow SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- Phase example: `docs/development/current/main/phases/phase-29bq/README.md`
- Box registry (code-side): `src/mir/builder/control_flow/plan/REGISTRY.md`
---

# Reject→Handoff SSOT + Gap Taxonomy (for Lego BoxCount)

This doc makes “where to add the next lego box” mechanical.
It prevents the common failure mode: a box prints `reject:<reason>` but still returns `Some`, causing Facts→Lower mismatch and confusing multi-AI debugging.

## 1) Reject→Handoff SSOT (minimal)

Rule: any `reject:<reason>` must imply one of:
- **handoff**: return `Ok(None)` from the *box entry* (so the next box can try)
- **fail-fast**: return `Freeze::contract(...)` (planner_required only; no silent fallback)

### Loop(cond) family (Phase 29bq)

- `loop_cond_continue_only`
  - `reject: return_in_body` → handoff: `loop_cond_continue_with_return` (BoxCount; sibling box)
  - `reject: continue_if_with_else` → handoff: `loop_cond_break_continue` (if it is break/continue capable) else next box
  - `reject: top_level_exit_stmt` → fail-fast (contract violation inside “continue-only” recipe)

- `loop_cond_break_continue` (and `cluster{3,4,5}`)
  - `reject: continue_only` → handoff: `loop_cond_continue_only` (or sibling)
  - `reject: return_in_body` → handoff: only if return is in supported “exit-if” shape; otherwise **out-of-scope**
  - `reject: nested_loop_count ...` → handoff: `clusterN` box (BoxCount)
  - `reject: exit_allowed_recipe_build_failed` → out-of-scope (ExitAllowed recipe build failure; update recipe SSOT or accept shape)
  - `reject: unsupported_stmt kind=Program/ScopeBox` → BoxShape: update container flatten (observation SSOT)

Notes:
- “extended mode” does not mean “accept any return”; it means “accept return only in the explicitly documented exit-if shapes”.

## 2) Gap taxonomy (what the next lego should be)

When a canary/log shows `planner returned None` or repeated `reject:<reason>`, classify the gap first:

- **Gap A: Candidate missing**
  - Symptom: `candidate_finalize result=none` (`[plan/trace]`)
  - Action: BoxCount: add one minimal box (Facts) + fixture + fast gate.

- **Gap B: Candidate present but not taken**
  - Symptom: `candidate_finalize result=some ...` but `try_take_planner ... result=skip`
  - Action: BoxShape: fix Recipe/PlanRuleId handoff mapping (no new acceptance shapes).

- **Gap C: Candidate ambiguous**
  - Symptom: `candidate_finalize result=ambiguous`
  - Action: BoxShape: clarify exclusivity/priority; avoid “two boxes accept the same shape”.

- **Gap D: Observation mismatch**
  - Symptom: Facts accepts but StepTree/extractor/parity disagrees (panic/freeze around “parity mismatch”)
  - Action: BoxShape: update observation SSOT (flatten Program/ScopeBox, shared walker, etc.) in the same commit.

## 3) Where to record updates

- Update this doc whenever you introduce a new `reject:<reason>` or a new handoff target.
- Update the box README (local scope SSOT) for the same reason/target pair.

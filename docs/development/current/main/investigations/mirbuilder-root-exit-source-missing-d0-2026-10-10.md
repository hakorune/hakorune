# MIRBUILDER-ROOT-EXIT-SOURCE-MISSING-D0

Status: selected design stop; no implementation decision yet
Date: 2026-10-10
Scope: first-stop census of the unchanged SizeClassBox published-view probe
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-borrowed-mul-static-entry-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home.rs
  - src/mir/normal_callable_semantic_package/ordinary_new_normal_return_projection.rs

## Observed boundary

After the completed scalar incoming cohort and source-only Mul entry S0,
the unchanged `lang/src/hako_alloc/memory/size_class_box.hako` published-view
probe (with protected, uncommitted Eq WIP in a detached diagnostic checkout)
stops at `ordinary-new/local-commit/root-exit-source-missing`. The prior
`borrowed-mul/source-only-entry` first stop is no longer reached. This is a
red integration observation, not a published-view PASS. Probe log:
`/tmp/hako-static-scalar-eq-probe4.log`.

`validate_root_home_exit` reaches this refusal only after object-return
construction readiness, Completion root flow and all-exits readiness. It
requires `normal_exit_projection_v1(owner, expected_exit)`, which may be
missing because the root-flow exit row, terminal relation or verified Normal
return disposition is absent. The freeze string alone does not identify which
authority is missing; do not implement from it.

## Next read-only decision work

1. Identify the exact owner, exit site, terminal source and absent projection
   component in this unchanged probe. Keep the protected Eq WIP separate.
2. Locate the canonical source authority and issuer for that component, the
   intended Normal/Fault and Home semantics, and the production caller to
   switch. Check whether this is the same earlier root-exit stop or a distinct
   exit with the same refusal.
3. Record one bounded Decision with positive/negative acceptance before any
   implementation. Preserve independent final MIR and physical checks.

No Bool physical result, whole SizeClassBox publication, or mimalloc-lite EXE
acceptance is claimed by the preceding S0.

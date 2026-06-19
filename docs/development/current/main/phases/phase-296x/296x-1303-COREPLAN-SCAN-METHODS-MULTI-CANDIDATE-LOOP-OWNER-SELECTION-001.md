---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the next owner for the scan-methods loop blocker after the
  ambiguous loop-var freeze moved to a multi-candidate loop shape.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1302-COREPLAN-SCAN-LOOP-COMMA-CLOSE-OWNER-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1289-COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1288-COREPLAN-LOOP-RESOLVER-SHADOW-001.md
---

# COREPLAN-SCAN-METHODS-MULTI-CANDIDATE-LOOP-OWNER-SELECTION

## Decision

The current blocker is not a PHI SSOT issue and not a safe one-line
`generic_loop_v1` candidate preference patch.

The failing function is:

```text
ParserBox.static_const_bitand/2
```

The visible loop shape is:

```hako
loop(a > 0 || b > 0) {
  ...
  a = a / 2
  b = b / 2
  bit = bit * 2
}
```

`generic_loop_v1` currently sees both `a` and `b` as valid loop-var
candidates and freezes under strict/planner_required:

```text
[plan/freeze:ambiguous] multiple loop_var candidates matched
```

Two drive-by implementations were rejected during local investigation:

```text
1. treating ambiguity as Ok(None)
   result=compile progressed, then ParserBox import / focused gate timed out or stack-overflowed

2. selecting the preferred left-side candidate
   result=unit-level extraction can pass, but the focused gate still times out/stack-overflows
```

Therefore, the next owner is a small owner-selection row for multi-candidate
loop semantics, not immediate lowering.

## B-lite Boundary

The current B-lite code remains a legacy registry observer:

```text
legacy_matched
legacy_effective
legacy_suppressed
legacy_selected
```

It is not yet an independent semantic resolver because its decision is still
derived from the existing registry candidate set. Do not promote it to route
selection.

The desired split remains:

```text
legacy_registry_observer.rs
  observe legacy matched/effective/suppressed/actual-selected routes

loop_resolver.rs
  frozen loop evidence -> Allow/Deny
  does not read ENTRIES or collect_candidates

loop_resolution_diagnostics.rs
  compare legacy observation with resolver decision
  feedback_to_resolver=0
```

## Selected Next Rows

```text
COREPLAN-MULTI-CANDIDATE-LOOP-OWNER-SELECTION-001
```

Purpose:

```text
Decide whether `a > 0 || b > 0` with independent updates is owned by:
  A. a conservative multi-carrier generic-loop recipe
  B. LoopCondBreak / flowbox adoption
  C. a source-level selfhost rewrite (rejected unless language semantics require it)
  D. deferred unsupported shape with explicit fixture expectation
```

Required before implementation:

```text
minimal fixture without ParserBox import
focused evidence for compile route and runtime behavior
no route-name / function-name special case
no preferred-candidate lowering without runtime proof
```

Separate cleanup row:

```text
COREPLAN-LOOP-ROUTE-RETIRE-001
target=registry_candidate_suppression
first_branch=loop_cond_continue_only_redundant_suppression
```

This cleanup is still valid but must not be confused with the active
multi-candidate loop blocker.

## Stop Lines

```text
do not select a loop-var candidate by variable name
do not lower multi-candidate loops through generic_loop_v1 without runtime proof
do not treat B-lite legacy observer parity as independent resolver proof
do not add route suppression to force the focused fixture through
do not rewrite ParserBox.static_const_bitand as a workaround
```

## Report

```text
output_contract=coreplan-scan-methods-multi-candidate-loop-owner-selection-v0
active_fixture=selfhost_blocker_scan_methods_loop_min
observed_function=ParserBox.static_const_bitand/2
failure=ambiguous_loop_var_candidates
phi_ssot_reopened=0
preferred_candidate_patch_rejected=1
ok_none_patch_rejected=1
b_lite_shadow_is_legacy_observer=1
resolver_selection_owner_enabled=0
next_task=COREPLAN-MULTI-CANDIDATE-LOOP-OWNER-SELECTION-001
summary=ok
```

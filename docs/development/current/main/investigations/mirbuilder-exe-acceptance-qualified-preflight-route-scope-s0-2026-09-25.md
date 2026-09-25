# MIRBUILDER-EXE-ACCEPTANCE-QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table +
  mir-call-d1b-main-raw-qualified-method-acceptance-d0-2026-09-21.md
  (qualified route scope = qualified receivers only)

## Slice

`src/mir/builder/normal_callable_semantic_loan_port/main_root.rs`:
narrowed the canonical-route diversion predicate (~:229) from
`method_calls().next().is_some()` to
`method_calls().any(|(_, call)| call.receiver() ==
ResolvedMethodCallReceiverSourceV1::QualifiedUnbound)`; added the
`ResolvedMethodCallReceiverSourceV1` import.

Lexical-only app-mains (e.g. `local pair = new Pair(10,20);
pair.sum()`) return to `inner.lower_body` — the lifecycle/ordinary_new
route that is their design-sanctioned owner
(MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-D1). Mixed mains still
route canonical and fail fast on the lexical call — no weakening.

## Fail-fast boundary

`capability.rs:734` and the `[mir/callable-main/qualified-preflight]`
wrap unchanged; the route still rejects out-of-scope shapes.

## Evidence

- `cargo check --profile quick` clean; `git diff --check` clean.
- Regression pin added:
  `normal_default_pipeline_tests.rs::lexical_receiver_method_call_main_stays_off_qualified_route`
  compiles `new Pair(10,20); return pair.sum()` end-to-end through
  `compile_normal_with_published` — green.
- Dedicated guard row registered: `mirbuilder-qualified-route-scope`
  in `tools/checks/guard_rows.toml` ->
  `tools/checks/mirbuilder_qualified_route_scope_guard.sh` (predicate
  must use `QualifiedUnbound`; old `next().is_some()` diversion is a
  forbidden regression; pin test must exist). `run_row_guard.sh --only`
  green.
- Focused smoke (fresh quick binary):
  `typed_object_method_min_exe` PASS — qualified-preflight `New`
  rejection gone.
- Full suite rerun (`real-apps-exe-boundary`, quick binary):
  **3 pass / 8 fail**. Moved boundaries classified on the parent D0
  card: `birth_param_min` now dies at
  `ordinary-new/local-commit/root-call-entry-missing` (real
  untyped-storage boundary) and `binary_trees` moved from
  `callable-loop/facts-absent` to
  `ordinary-new/birth-global-legacy-stopped` (its `new` statement now
  hits the ordinary-new lane's named stop before the loop Facts gap —
  boundary move, both terms remain fail-fast named stops).

## Exit

- [x] Predicate narrowed; lexical-only mains on lifecycle route.
- [x] Pins + focused suite green; new reds classified vs baseline
      (8 remaining classes recorded on the D0 card; no silent red).
- [x] Guard row + card/pointer/workstream synced; commit+push.

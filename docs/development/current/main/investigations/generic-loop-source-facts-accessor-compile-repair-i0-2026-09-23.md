---
Status: closed__2026-09-23__build_green_front_named_terminal
Task: GENERIC-LOOP-SOURCE-FACTS-ACCESSOR-COMPILE-REPAIR-I0
Date: 2026-09-23
Parent: generic-legacy-observation-front-g0-premise-recheck-2026-09-23.md
NextCard: generic-post-selection-static-call-site-identity-d0-2026-09-23.md
Implementation permission: one behavior-preserving source projection repair in the existing Generic source-facts owner, then one pinned G0 smoke; no fixture, route, authority, or production-selection changes.
---

# Generic source-facts accessor compile repair I0

## Six-line brief

```text
Decision: repair the current compiler red that prevents the authorized one-case G0 premise recheck.
Source authority + canonical issuer: existing CallableGenericLoopSourceRouteAdmissionV1 evidence owns the pre-effect receipt; existing source-facts/Recipe view projects that evidence.
Non-authority: the #[cfg(test)] receipt convenience accessor is not a production API or a new issuer; planner/shadow tags alone are not LoopReached.
Fail-fast boundary: the quick-profile compiler build must pass before the exact G0 wrapper runs; preserve route selection, evidence identity, and all failure behavior.
Smallest next slice: in normal_callable_loop_source_facts/generic.rs, have production views borrow pre_effect through route_admission().evidence(); retain the test-only helper for tests.
Non-claims: no new semantic receipt, Generic route/disposition, Recipe support, production switch, fallback, parity, or legacy deletion.
```

## Observed current-change red

At `b48ecd9fb2c20be0d6eb4e7eec35b00f59300941`, this command failed:

```text
CARGO_BUILD_JOBS=4 cargo build --profile quick --bin hakorune
exit 101; rustc 1.89.0
E0599: no method named `pre_effect` for CallableGenericLoopSourceFactsReceiptV1
src/mir/builder/normal_callable_loop_source_facts/generic.rs:424, 463, 491, 492
```

The method at `generic.rs:207` is guarded by `#[cfg(test)]`, while the
production projections call it. The access-path change and these call sites
were introduced together by `089ffba9aa` (`feat(mir): admit source-backed
generic loop overlap`); the parent representation held the pre-effect value
directly. This is a current-change compile failure, not a baseline test red or
an observation result from the G0 smoke.

## Bounded repair

Keep the test-only convenience accessor under `#[cfg(test)]`. In each affected
production view, read the same pre-effect receipt from the already selected
`route_admission().evidence()` owner. Do not remove `cfg(test)` to widen the
receipt API, copy or reissue evidence, add a fallback/default, or change which
route is selected. This is BoxShape/source projection repair only.

The same bounded series then runs only:

```text
CARGO_BUILD_JOBS=4 cargo build --profile quick --bin hakorune
NYASH_BIN=target/quick/hakorune HAKO_BIN=target/quick/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/generic_loop_continue_strict_shadow_vm.sh
```

The smoke is pinned to `generic_loop_continue_strict_shadow_vm`, fixture
`apps/tests/phase29ca_generic_loop_continue_min.hako`, and profile
`vm-strict-planner-direct-v1`. Do not run the corpus or another backend.

## Acceptance and handoff

- The quick binary build passes with the existing source authority intact.
- The pinned wrapper runs against that freshly built binary; record SHA,
  command, exit status, and first owner/terminal in the G0 card.
- A passing front means exit `4` plus the accepted Loop tag. A tag followed by
  nonzero exit is not a pass.
- A post-selection static-call terminal whose caller/site is not correlated is
  not a passing front or an in-loop GenericLoop failure. It selects only the
  source-identity design stop; no route observation follows.
- Run `bash tools/checks/current_state_pointer_guard.sh` and
  `git diff --check`; update the G0 card and pointer to the observed next
  action. No full acceptance suite or CI wait is required here.

## Closeout evidence

- Source repair landed at `fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f`:
  production projections now borrow `pre_effect` from the existing
  `route_admission().evidence()` owner; the test-only accessor and receipt
  identity remain unchanged.
- `rustfmt --check src/mir/builder/normal_callable_loop_source_facts/generic.rs`
  and `git diff --check` passed.
- `CARGO_BUILD_JOBS=4 cargo build --profile quick --bin hakorune` passed on
  that source, Rust 1.89.0, in 3m08s. The build reported 1,683 warnings; this
  task made no warning-suppression or warning-cohort change.
- At the same SHA, the pinned wrapper exited 1 (not the expected 4). It was
  invoked through `bash` at `2026-09-23T14:13:39+09:00`; its first named terminal was
  `[freeze:contract][static-call/legacy-fallback-retired] owner=StringHelpers
  method=to_i64 arity=1`. Loop planner/shadow tags appeared earlier, but the
  diagnostic has no caller or `SourceExprSiteV1`; the worker audit did not
  correlate this terminal with the printed Loop function or the known
  `StringHelpers.int_to_str/1` site.
- Therefore this repair row closes its build obligation only. The pinned front
  remains unaccepted and is handed to the exact source-identity design stop;
  there is no LoopRecipe, publication, production-switch, or fallback claim.

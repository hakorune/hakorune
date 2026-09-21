---
Status: design_stop__2026-09-21__SelectedBoundedWarningCohort__ReadyForFastTransition
Task: MIRBUILDER-WARNING-DYNAMIC-OPERATOR-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i69-2026-09-21.md
Implementation permission: false until pointer transition commit
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I70
---

# MirBuilder dynamic operator test facade I0

## Six-line brief

```text
Decision: move the unused operator contract imports into tests.rs and remove
  only those parent facade items.
Source authority + canonical issuer: dynamic_operator_contract issuer.rs/model.rs.
Non-authority: the parent mod.rs facade, warning-count guesses, or route code.
Fail-fast boundary: a production consumer, unresolved test import, or changed
  operator result stops the slice.
Smallest next slice: direct-import the selected issuer/model items in tests.rs,
  remove those parent re-exports, and run the two fixed quick gates.
Non-claims: no operator semantic change, no route change, no suppression, and
  no cleanup outside the selected test facade.
```

## Evidence and acceptance

I69 refreshed the fixed baseline at lib **1,753** / lib-test **557**. The source
census is finite and leaves the used domain/family/result/value exports in the
parent module. The fast slice must run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

All dynamic-operator tests must compile and pass; no selected item may remain
as a parent facade or reappear in another owner layer.

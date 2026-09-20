---
Status: closed__2026-09-21__WarningBaselineRefreshI1__LoopScheduleSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I1
Date: 2026-09-21
Parent: mirbuilder-warning-unused-import-array-text-callee-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-UNUSED-IMPORT-LOOP-SCHEDULE-I0
---

# MirBuilder warning baseline refresh I1

## Six-line brief

```text
Decision: refresh both warning surfaces after the completed import move before
  selecting another cohort; the first reduction is now recorded separately.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the existing warning inventory/parser, with test-only uses made explicit.
Non-authority: historical counts, cargo-fix, warning-driven semantic edits,
  blanket allow, visibility widening, or a single lib-only observation.
Fail-fast boundary: any new warning family, failure name, command drift, or
  unclassified import/test reference stops selection.
Smallest next slice: compare current lib/lib-test inventories against the
  1,844/563 post-I0 receipt and select at most one next caller-zero cohort.
Non-claims: no broad warning cleanup, dead-code deletion, suppression, or
  production route change.
```

## Boundary and current receipt

The fixed surfaces are `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4`. The post-I0 receipt is
lib=1,844 and lib-test=563, both exit 0. The completed change moved `Callee`
from a production import to an explicit test import and is outside this new
census.

I1 must classify lint, file:line, owner, and production/test/compat/generated
role again. It may select one import cohort only when both surfaces prove the
old import path is unused or the test-only dependency is explicitly moved.
Dead-code and private-interface rows remain with their semantic owners.

The post-I0 receipt was reproduced at the next head: lib=1,844 and
lib-test=563 before the loop-schedule move. The selected cohort was the
production-only `VerifiedCallableSemanticLoopBindingScheduleV1` import; its
test dependency was already explicit in the test file. The row is closed by
the child card with lib=1,843 and lib-test=563.

## Acceptance and stop

I1 closes with two reproducible command receipts, stable failure-name
classification, and the one selected cohort. No broader code, fixture,
`allow`, or baseline manifest change is authorized by this card.

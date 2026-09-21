---
Status: fast__2026-09-21__SelectedBoundedWarningCohort__ExecuteTestScope
Task: MIRBUILDER-WARNING-LOOP-FAMILY-ADMISSION-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i75-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the six LoopFamilyAdmission re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I76
---

# MirBuilder loop family admission test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for six caller-zero loop family
  admission symbols while retaining their existing test facade.
Source authority + canonical issuer: loop_route_policy/family_admission.rs.
Non-authority: parent-module reachability, warning counts, or a new family
  selector/admission owner.
Fail-fast boundary: any non-test caller, missing test import, changed owner, or
  focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run admission /
  selector tests plus the quick library check.
Non-claims: no family admission semantic change, no selector change, no
  production route change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

The six symbols are defined in `family_admission.rs`. Production observation
and selector modules import that owner module directly. Parent-facade imports
are confined to `family_admission_tests.rs` and `family_selector_tests.rs`.
Keep those tests and scope the parent re-export to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib family_admission -- --nocapture
cargo test --profile quick --lib family_selector -- --nocapture
```

Both focused suites must pass. Record the warning delta, preserve known
baseline classification, and run `cargo fmt --check`, `git diff --check`, and
the current-state pointer guard before commit/push.

---
Status: closed__2026-09-21__DynamicOperatorTestFacade__OwnerDirectImports
Task: MIRBUILDER-WARNING-DYNAMIC-OPERATOR-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i69-2026-09-21.md
Implementation permission: true for the selected issuer/model test imports only
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

## Closeout evidence

Commit `92a7a542c4` moved the selected issuer and model imports into
`dynamic_operator_contract/tests.rs` and removed their parent re-exports. The
used domain/family/result/value exports remain in the parent module. The source
census shows no selected item outside its canonical issuer/model owners and the
test module.

The fixed gates passed sequentially: lib generated **1,751** warnings and
lib-test generated **557** warnings. The four focused dynamic-operator tests
passed (`4 passed, 0 failed`). `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard are green. No operator semantics or route changed.

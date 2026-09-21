---
Status: design_stop__2026-09-21__SelectedBoundedWarningCohort__ReadyForFastTransition
Task: MIRBUILDER-WARNING-IF-JOINSIG-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i70-2026-09-21.md
Implementation permission: false until pointer transition commit
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I71
---

# MirBuilder If JoinSig test facade I0

## Six-line brief

```text
Decision: move four JoinSig test imports to their two owner test modules and
  remove only the corresponding parent facade exports.
Source authority + canonical issuer: if_recipe_contract/join_sig.rs.
Non-authority: the parent mod.rs facade, warning-count guesses, or physicalizer
  test scaffolding as a semantic authority.
Fail-fast boundary: a production consumer, unresolved test import, or changed
  If recipe behavior stops the slice.
Smallest next slice: direct-import the four join_sig items in the two test
  modules, remove their parent re-exports, and run the fixed quick gates.
Non-claims: no JoinSig semantics, no physicalizer route change, no suppression,
  and no cleanup outside these four facade items.
```

## Evidence and acceptance

I70 refreshed the fixed baseline at lib **1,751** / lib-test **557**. The source
census is finite and shows only two test consumers. The fast slice must run
sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

The if-contract and physicalizer focused tests must compile and pass. No
selected item may remain as a parent facade or reappear in another owner layer.

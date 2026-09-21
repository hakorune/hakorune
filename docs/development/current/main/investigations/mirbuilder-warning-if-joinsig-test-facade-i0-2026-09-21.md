---
Status: closed__2026-09-21__IfJoinSigElaboratorFacade__OwnerDirectImport
Task: MIRBUILDER-WARNING-IF-JOINSIG-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i70-2026-09-21.md
Implementation permission: true for the one JoinSig elaborator test import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I71
---

# MirBuilder If JoinSig test facade I0

## Six-line brief

```text
Decision: move the caller-zero JoinSig elaborator import to its owner test
  module and remove only the corresponding parent facade export.
Source authority + canonical issuer: if_recipe_contract/join_sig.rs.
Non-authority: the parent mod.rs facade, warning-count guesses, or physicalizer
  test scaffolding as a semantic authority.
Fail-fast boundary: a production consumer, unresolved test import, or changed
  If recipe behavior stops the slice.
Smallest next slice: direct-import IfJoinSigElaboratorV1 in tests.rs, remove its
  parent re-export, and run the fixed quick gates.
Non-claims: no JoinSig semantics, no physicalizer route change, no suppression,
  and no cleanup outside the selected facade item.
```

## Evidence and acceptance

I70 refreshed the fixed baseline at lib **1,751** / lib-test **557**. The source
census is finite and shows one parent-facade test consumer; the other JoinSig
items remain on the sibling physicalizer boundary. The fast slice must run
sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

The if-contract focused tests must compile and pass, and no selected item may
remain as a parent facade or reappear in another owner layer. Physicalizer
JoinSig imports remain unchanged.

## Closeout evidence

Commit `760e852155` moved `IfJoinSigElaboratorV1` directly into the
if-contract test module and removed its parent re-export. The three JoinSig
items required by the sibling physicalizer test remain intentionally unchanged.
The source census confirms the elaborator is now used only by its canonical
owner and the owner test.

The fixed gates passed sequentially: lib generated **1,751** warnings and
lib-test generated **557** warnings; the grouped diagnostic stayed at the same
count while one import item was removed. The if-contract focused suite passed
(`10 passed, 0 failed`). `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard are green. No If recipe or physicalizer semantics
changed.

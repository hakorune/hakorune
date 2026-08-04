---
Status: Planned; gated by the nested If design stop
Date: 2026-08-04
Parent: joinir-if-recipe-nested-one-level-design-stop-2026-08-04.md
Decision: if accepted, execute only depth-one nested pure fallthrough D0,
  D1 ownership census, and D2 parity/candidate-abort evidence
---

# One-level Nested If — bounded execution task

This card is not permission to widen the one-If recipe or to add recursive
production routing. The parent design stop owns the exact shape and stop
conditions.

## D0 — named nested facts and composition contract

Add a separate nested profile/product for exactly one outer and one inner
explicit-else If over one shared i64 binding. Keep the current one-If profile
immutable.

Required evidence:

* deterministic preorder facts and source claims for both If nodes;
* child artifact and outer artifact carry only portable identities;
* inner and outer JoinSig rows compose with the inner merge as the outer then
  edge;
* depth greater than one, implicit nested else, unsupported child operation,
  missing transfer/read, path mismatch, and multiple-binding fixtures reject
  before Builder effects.

## D1 — caller and owner census

Prove one nested producer/mapper/composition chain and one selected physicalizer
caller. Existing CanonicalCfg, Binding SSA, and PhiTxn remain sole physical
owners. New route, retry, transaction, and PHI/SSA owner counts must be zero.

## D2 — parity and candidate abort

Use the exact source fixture:

```text
local x = 0
if x < 10 { if x < 5 { x = 1 } else { x = 2 } } else { x = 3 }
return x
```

Prove results 1/2/3, two PHIs, exact predecessor/value correspondence, and
interpreter parity. Inject late failure after both nested PHIs using the
existing candidate seam; prove live state is unchanged and fresh reuse works.
If that fault point does not exist, stop and open a design row instead of
adding a new test-only fault mechanism.

## Required gates

Use the focused resolved profile, IfRecipe contract, canonical-session, and
candidate-abort suites plus:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The production-shaped interpreter fixture must run with the existing
`vm-reference` feature. Keep every touched source/test file below 800 lines.

## Stop conditions

Stop and return to the design card if the implementation needs depth greater
than one, a new CFG/PHI/SSA owner, a second physicalizer, retry/fallback, raw
AST/name lookup, ownership rules, or a new fault/transaction API.

---
Status: queued after R1; do not execute early
Date: 2026-08-08
Decision: parser-owned ExplicitSource issuance and duplicate/site seal
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R2

## Goal

Make ordinary, interface, and static Box parser branches issue exact
`ExplicitSource` inventory rows while parsing declarations once. R2 is the
first row allowed to claim parser-owned source order, duplicate rejection, and
exact Box/member source sites.

## Required work

```text
one atomic parser batch per Box declaration
ordered member ordinal from the parser walk
exact Box declaration site and method declaration site
duplicate method rejection before AST publication
ordinary/interface/static branches use the same inventory contract
selected build-gate and generated rows remain closed for R3
```

Do not reconstruct order or sites from the R1 compatibility inventory. Raw
`CompatibilityOnly` rows cannot be upgraded; the parser must issue fresh
source-backed rows from the as-written declaration walk.

## Acceptance

```text
positive ordered ordinary/interface/static Box fixtures
direct duplicate rejection with both diagnostic sites
invalid/non-function compatibility fixtures remain confined to R1 tests
no HashMap/name-sort source authority
no resolver, CallableContract issuer, target, Recipe, Builder, or runtime claim
all touched files < 800 lines
```

The implementation commit updates the frontend parser/AST owner README,
`docs/reference/language/callable-contracts.md` only for the exact landed
frontend status, this card's receipt, and `CURRENT_STATE.toml`. R3 is selected
only after these tests and the current pointer guard are green.

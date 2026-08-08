---
Status: active — revised after whole-owner audit
Date: 2026-08-08
Decision: parser-owned ExplicitSource issuance and duplicate/site seal
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R2

## Goal

Build the shared direct-method issuance substrate, then cut over only the
interface and static Box parser branches to fresh `ExplicitSource` rows.
Ordinary Box remains a deliberate non-claim until R3 can replace its mixed
direct/generated/selected/postfix owner atomically.

## Required work

```text
shared parser-local PendingExplicitMethodV1
method parse -> pending postfix mutation -> one inventory commit
interface/static ordered member ordinal from the parser walk
exact parser structural site + available token line/column
duplicate rejection with first and duplicate diagnostic sites
metadata-preserving consuming declaration transform for build_cfg pruning
ordinary/selected/generated/delegate authority remains closed for R3
```

Do not reconstruct order or sites from the R1 compatibility inventory. Raw
`CompatibilityOnly` rows cannot be upgraded. Do not expose inventory mutation
or `get_mut`: postfix `catch`/`cleanup` mutates only an unpublished pending
method and commits it once at the next member or Box end.

`build_cfg` may transform declaration bodies only through a consuming API that
preserves name, ordinal, provenance, structural site, and diagnostic span.
The current inventory -> HashMap -> CompatibilityOnly round trip is deleted in
this row. A transformed declaration whose name changes is rejected.

## Acceptance

```text
positive ordered interface/static Box fixtures
interface/static duplicate rejection with both diagnostic sites
newline-separated postfix keeps one identity and its original site
build_cfg pruning preserves order/provenance/site/span
ordinary ExplicitSource issuance remains exactly zero
invalid/non-function compatibility fixtures remain confined to R1 tests
no HashMap/name-sort source authority
no resolver, CallableContract issuer, target, Recipe, Builder, or runtime claim
all touched files < 800 lines
```

The implementation commit updates the frontend parser/AST owner README,
`docs/reference/language/callable-contracts.md` only for the exact landed
frontend status, this card's receipt, and `CURRENT_STATE.toml`. R3 is selected
only after these tests and the current pointer guard are green.

## Why ordinary is R3

The ordinary Box owner currently combines direct methods, generated property
methods, selected build-gate methods, and postfix mutation in one `HashMap`.
Opening only its direct rows here would require a dual ledger or a false
`CompatibilityOnly` downgrade. Both are forbidden. R3 replaces that owner and
its generated/selected transactions as one BoxShape cutover.
